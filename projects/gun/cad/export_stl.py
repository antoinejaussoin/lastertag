#!/usr/bin/env python3
"""Export approximate STLs for the gun shell when OpenSCAD is unavailable.

Geometry mirrors the named dimensions in gun.scad (boxes / cylinders only).
Prefer `make` with OpenSCAD for the full parametric model; this is a printable
fallback so the repo ships meshes.
"""
from __future__ import annotations

import math
import struct
from pathlib import Path

STL_DIR = Path(__file__).resolve().parent / "stl"

# Dimensions shared with gun.scad
FRAME_W = 42.0
SLIDE_W = 32.0
SLIDE_L = 186.0
MAG_W = FRAME_W - 2 * 2.4 - 1
MAG_D = 28.0
MAG_H = 105.0
LIPO_L, LIPO_W, LIPO_H = 36.0, 31.0, 7.0
OLED_PCB = 28.0
OLED_GLASS_L, OLED_GLASS_W = 24.0, 14.0
BARREL_OD, BARREL_ID, LED_D = 14.0, 8.0, 5.5
PICO_L, PICO_W, PICO_H = 52.0, 21.5, 12.0
USB_L, USB_W, USB_H = 16.0, 10.0, 8.0
BTN = 6.5
PIN_D = 2.2
WALL = 2.4


def tri(v0, v1, v2):
    ax, ay, az = v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]
    bx, by, bz = v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]
    nx, ny, nz = ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx
    n = math.sqrt(nx * nx + ny * ny + nz * nz) or 1.0
    return (nx / n, ny / n, nz / n), v0, v1, v2


class Mesh:
    def __init__(self):
        self.faces = []

    def add_tri(self, a, b, c):
        self.faces.append(tri(a, b, c))

    def add_quad(self, a, b, c, d):
        self.add_tri(a, b, c)
        self.add_tri(a, c, d)

    def box(self, xmin, xmax, ymin, ymax, zmin, zmax):
        x0, x1, y0, y1, z0, z1 = xmin, xmax, ymin, ymax, zmin, zmax
        # bottom / top
        self.add_quad((x0, y0, z0), (x1, y0, z0), (x1, y1, z0), (x0, y1, z0))
        self.add_quad((x0, y0, z1), (x0, y1, z1), (x1, y1, z1), (x1, y0, z1))
        # sides
        self.add_quad((x0, y0, z0), (x0, y0, z1), (x1, y0, z1), (x1, y0, z0))
        self.add_quad((x1, y0, z0), (x1, y0, z1), (x1, y1, z1), (x1, y1, z0))
        self.add_quad((x1, y1, z0), (x1, y1, z1), (x0, y1, z1), (x0, y1, z0))
        self.add_quad((x0, y1, z0), (x0, y1, z1), (x0, y0, z1), (x0, y0, z0))

    def hollow_box(self, outer, inner):
        """outer/inner = (xmin,xmax,ymin,ymax,zmin,zmax). Simple shell via six slabs."""
        ox0, ox1, oy0, oy1, oz0, oz1 = outer
        ix0, ix1, iy0, iy1, iz0, iz1 = inner
        # floor / ceiling
        self.box(ox0, ox1, oy0, oy1, oz0, iz0)
        self.box(ox0, ox1, oy0, oy1, iz1, oz1)
        # walls
        self.box(ox0, ix0, oy0, oy1, iz0, iz1)
        self.box(ix1, ox1, oy0, oy1, iz0, iz1)
        self.box(ix0, ix1, oy0, iy0, iz0, iz1)
        self.box(ix0, ix1, iy1, oy1, iz0, iz1)

    def cylinder_z(self, x, y, z0, z1, r, segments=32, solid=True):
        pts0, pts1 = [], []
        for i in range(segments):
            a = 2 * math.pi * i / segments
            pts0.append((x + r * math.cos(a), y + r * math.sin(a), z0))
            pts1.append((x + r * math.cos(a), y + r * math.sin(a), z1))
        c0, c1 = (x, y, z0), (x, y, z1)
        for i in range(segments):
            j = (i + 1) % segments
            if solid:
                self.add_tri(c0, pts0[j], pts0[i])
                self.add_tri(c1, pts1[i], pts1[j])
            self.add_quad(pts0[i], pts0[j], pts1[j], pts1[i])

    def tube_z(self, x, y, z0, z1, r_out, r_in, segments=32):
        self.cylinder_z(x, y, z0, z1, r_out, segments)
        # invert inner by flipping winding via reverse order centers — approximate
        # by not capping; add inner wall only
        pts0o, pts1o, pts0i, pts1i = [], [], [], []
        for i in range(segments):
            a = 2 * math.pi * i / segments
            ca, sa = math.cos(a), math.sin(a)
            pts0o.append((x + r_out * ca, y + r_out * sa, z0))
            pts1o.append((x + r_out * ca, y + r_out * sa, z1))
            pts0i.append((x + r_in * ca, y + r_in * sa, z0))
            pts1i.append((x + r_in * ca, y + r_in * sa, z1))
        # rebuild clean tube
        m = Mesh()
        for i in range(segments):
            j = (i + 1) % segments
            # outer
            m.add_quad(pts0o[i], pts0o[j], pts1o[j], pts1o[i])
            # inner (inward)
            m.add_quad(pts0i[j], pts0i[i], pts1i[i], pts1i[j])
            # rings
            m.add_quad(pts0o[i], pts0i[i], pts0i[j], pts0o[j])
            m.add_quad(pts1o[j], pts1i[j], pts1i[i], pts1o[i])
        self.faces.extend(m.faces)

    def write_stl(self, path: Path, name: str):
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open("wb") as f:
            header = name.encode("ascii", "replace")[:80].ljust(80, b"\0")
            f.write(header)
            f.write(struct.pack("<I", len(self.faces)))
            for n, a, b, c in self.faces:
                f.write(struct.pack("<3f", *n))
                f.write(struct.pack("<3f", *a))
                f.write(struct.pack("<3f", *b))
                f.write(struct.pack("<3f", *c))
                f.write(struct.pack("<H", 0))


def frame_half(side: int) -> Mesh:
    """side -1 left (x<=0), +1 right (x>=0). Simplified clamshell half."""
    m = Mesh()
    x0, x1 = (-FRAME_W / 2, 0.0) if side < 0 else (0.0, FRAME_W / 2)
    # Grip
    m.hollow_box(
        (x0, x1, 0, 55, 0, 95),
        (x0 + WALL if side < 0 else x0, x1 - WALL if side > 0 else x1, WALL, 55 - WALL, WALL, 95 - WALL),
    )
    # Dust cover
    m.hollow_box(
        (x0 + 2, x1 - 2, 55, 175, 85, 110),
        (
            x0 + 2 + WALL if side < 0 else x0 + 2,
            x1 - 2 - WALL if side > 0 else x1 - 2,
            55 + WALL,
            175 - WALL,
            85 + WALL,
            110 - WALL,
        ),
    )
    # Pico shelf (solid pad)
    m.box(x0 + WALL, x1 - WALL if side > 0 else x1, 2, 12, 45, 48)
    # Screw bosses (half cylinders as boxes)
    for y, z in [(5, 20), (5, 90), (35, 110), (90, 85), (140, 85), (165, 100), (55, 40), (20, 40)]:
        m.box(x0 + 1, x1 - 1, y - 3, y + 3, z - 3, z + 3)
    return m


def slide() -> Mesh:
    m = Mesh()
    m.hollow_box(
        (-SLIDE_W / 2, SLIDE_W / 2, 0, SLIDE_L, 110, 132),
        (-SLIDE_W / 2 + WALL, SLIDE_W / 2 - WALL, WALL, SLIDE_L - WALL, 110 + WALL, 132 - WALL),
    )
    # Optic hump
    m.hollow_box(
        (-SLIDE_W / 2 - 1, SLIDE_W / 2 + 1, 10, 10 + OLED_PCB + 8, 128, 142),
        (
            -OLED_PCB / 2,
            OLED_PCB / 2,
            14,
            14 + OLED_PCB,
            130,
            140,
        ),
    )
    # Window cut approximated by not filling glass — leave open by thinner top already
    return m


def trigger() -> Mesh:
    m = Mesh()
    m.box(-4, 4, 0, 20, -4, 4)
    m.box(-10, 10, -8, 0, 0, 8)
    # pivot hole as negative not possible — leave solid; user drills or uses OpenSCAD
    return m


def magazine() -> Mesh:
    m = Mesh()
    m.hollow_box(
        (-MAG_W / 2, MAG_W / 2, -MAG_D / 2, MAG_D / 2, 0, MAG_H),
        (-LIPO_W / 2, LIPO_W / 2, -LIPO_H / 2, LIPO_H / 2, 8, 8 + LIPO_L),
    )
    return m


def muzzle_collar() -> Mesh:
    m = Mesh()
    m.tube_z(0, 0, 0, 22, (BARREL_OD + 8) / 2, (BARREL_OD + 0.4) / 2)
    return m


def oled_bezel() -> Mesh:
    m = Mesh()
    outer = OLED_PCB + 6
    m.hollow_box(
        (-outer / 2, outer / 2, -outer / 2, outer / 2, 0, 4),
        (-OLED_GLASS_W / 2, OLED_GLASS_W / 2, -OLED_GLASS_L / 2, OLED_GLASS_L / 2, -0.1, 4.1),
    )
    return m


PARTS = {
    "frame_left": lambda: frame_half(-1),
    "frame_right": lambda: frame_half(1),
    "slide": slide,
    "trigger": trigger,
    "magazine": magazine,
    "muzzle_collar": muzzle_collar,
    "oled_bezel": oled_bezel,
}


def main():
    STL_DIR.mkdir(parents=True, exist_ok=True)
    for name, fn in PARTS.items():
        mesh = fn()
        out = STL_DIR / f"{name}.stl"
        mesh.write_stl(out, name)
        print(f"wrote {out} ({len(mesh.faces)} triangles)")


if __name__ == "__main__":
    main()

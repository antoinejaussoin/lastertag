// Laser-tag gun shell — Glock 17 Gen5 proportions, electronics-real packing.
// Units: millimetres. Vanilla OpenSCAD (no BOSL).
//
// Part select:
//   part = "preview" | "frame_left" | "frame_right" | "slide"
//        | "trigger" | "magazine" | "muzzle_collar" | "oled_bezel"
//
// Measure your OLED PCB and LiPo before a long print; tweak the named
// constants below if pockets are tight.

part = "preview";

// --- Glock 17 Gen5 starting envelope (then widened for parts) ---
overall_length = 202;
overall_height = 139;
frame_width    = 42;   // real ~34; widened for Pico + wiring
slide_width    = 32;   // real 25.5; widened for OLED PCB
slide_length   = 186;
barrel_length  = 114;
trigger_reach  = 70;   // trigger face from backstrap reference
wall           = 2.4;
clearance      = 0.35;
screw_d        = 2.2;  // clearance for M2
screw_boss     = 6;

// --- Electronics pockets (measure and edit) ---
pico_l = 52;   // 51 mm PCB + USB overhang allowance
pico_w = 21.5;
pico_h = 12;   // board + headers + wires
usb_cut_l = 16;
usb_cut_w = 10;
usb_cut_h = 8;
bootsel_d = 4.5;

oled_pcb_l = 28;
oled_pcb_w = 28;
oled_pcb_h = 6;
oled_glass_l = 24;
oled_glass_w = 14;
oled_tilt_deg = 12;

lipo_l = 36;
lipo_w = 31;
lipo_h = 7;    // 5 mm cell + foam
mag_outer_w = frame_width - 2 * wall - 1;
mag_outer_d = 28;
mag_outer_h = 105;

led_d = 5.5;
led_recess = 18;
barrel_id = 8;
barrel_od = 14;

btn_xy = 6.5;
btn_h  = 5;
trigger_pin_d = 2.2;

eps = 0.02;
$fn = 48;

module rounded_rect(size, r) {
    x = size[0]; y = size[1]; z = size[2];
    hull() {
        for (sx = [-1, 1], sy = [-1, 1])
            translate([sx * (x / 2 - r), sy * (y / 2 - r), 0])
                cylinder(h = z, r = r);
    }
}

module pico_keepout() {
    // Vertical in backstrap: USB toward heel (−Y), antenna toward beavertail (+Y)
    cube([pico_w + 2 * clearance, pico_l + 2 * clearance, pico_h + 2 * clearance], center = true);
}

module usb_keepout() {
    translate([0, -(pico_l / 2 + usb_cut_l / 2 - 1), 0])
        cube([usb_cut_w, usb_cut_l, usb_cut_h], center = true);
}

module oled_keepout() {
    cube([oled_pcb_w + 2 * clearance, oled_pcb_l + 2 * clearance, oled_pcb_h + 2 * clearance], center = true);
}

module led_keepout() {
    rotate([0, 90, 0])
        cylinder(h = led_recess, d = led_d + clearance, center = true);
}

module button_keepout() {
    cube([btn_xy + clearance, btn_xy + clearance, btn_h + clearance], center = true);
}

// Frame outline in X (width) × Y (length forward) × Z (height up)
// Origin: grip back heel, floor of mag well-ish; +Y toward muzzle, +Z up.
module frame_outer() {
    // Grip block
    translate([0, 18, 45])
        rounded_rect([frame_width, 55, 95], 4);
    // Dust cover / under-barrel
    translate([0, 95, 95])
        rounded_rect([frame_width - 4, 110, 28], 3);
    // Trigger guard loop (solid then hollowed)
    translate([0, 52, 28])
        rounded_rect([frame_width - 6, 42, 36], 3);
    // Beavertail / backstrap upper
    translate([0, 8, 100])
        rounded_rect([frame_width - 2, 28, 40], 3);
}

module frame_hollow() {
    // Mag well
    translate([0, 22, 42])
        cube([mag_outer_w - 2, mag_outer_d - 2, mag_outer_h], center = true);
    // Trigger guard opening
    translate([0, 55, 22])
        rounded_rect([frame_width, 28, 22], 6);
    // Pico cavity in backstrap (USB at heel)
    translate([0, 10, 55])
        rotate([8, 0, 0]) {
            pico_keepout();
            usb_keepout();
        }
    // Wire channel grip → barrel
    translate([0, 70, 88])
        cube([8, 90, 10], center = true);
    // IR nest cavity under muzzle
    translate([0, 150, 92])
        cube([frame_width - 10, 40, 18], center = true);
    // Barrel bore through dust cover
    translate([0, 100, 100])
        rotate([0, 90, 0])
            cylinder(h = barrel_length + 20, d = barrel_id, center = true);
    // LED recess at muzzle
    translate([0, 168, 100])
        led_keepout();
    // Trigger switch pocket
    translate([0, 48, 55])
        button_keepout();
    // Trigger pivot hole
    translate([0, 42, 62])
        rotate([0, 90, 0])
            cylinder(h = frame_width + 4, d = trigger_pin_d + clearance, center = true);
    // BOOTSEL access (left side when assembled — cut in both halves for symmetry)
    translate([-frame_width / 2 + 1, 6, 70])
        rotate([0, 90, 0])
            cylinder(h = wall + 4, d = bootsel_d);
    // Screw bosses tunnels
    for (p = screw_points())
        translate(p)
            cylinder(h = frame_width + 4, d = screw_d, center = true);
}

function screw_points() = [
    [0, 5, 20],
    [0, 5, 90],
    [0, 35, 110],
    [0, 90, 85],
    [0, 140, 85],
    [0, 165, 100],
    [0, 55, 40],
    [0, 20, 40]
];

module screw_bosses() {
    for (p = screw_points())
        translate(p)
            rotate([0, 90, 0])
                cylinder(h = frame_width - 2, d = screw_boss, center = true);
}

module frame_solid() {
    difference() {
        union() {
            frame_outer();
            screw_bosses();
        }
        frame_hollow();
        // Split plane chamfer helper: thin kerf for mating faces is handled by half()
    }
}

module frame_half(side) {
    // side = -1 left (negative X), +1 right
    difference() {
        intersection() {
            frame_solid();
            translate([side * (frame_width / 2 + 50) / 2, overall_length / 2, overall_height / 2])
                cube([frame_width / 2 + 50, overall_length + 40, overall_height + 40], center = true);
        }
        // Mating face relief
        translate([0, overall_length / 2, overall_height / 2])
            cube([clearance, overall_length + 50, overall_height + 50], center = true);
    }
}

module slide_body() {
    difference() {
        union() {
            translate([0, slide_length / 2 - 5, 118])
                rounded_rect([slide_width, slide_length, 24], 2);
            // Optic hump at rear
            translate([0, 28, 132])
                rotate([-oled_tilt_deg, 0, 0])
                    rounded_rect([slide_width + 2, oled_pcb_l + 8, 14], 2);
        }
        // OLED pocket
        translate([0, 28, 134])
            rotate([-oled_tilt_deg, 0, 0]) {
                oled_keepout();
                // Glass window
                translate([0, 0, 6])
                    cube([oled_glass_w, oled_glass_l, 10], center = true);
            }
        // Wire slot down into frame
        translate([0, 40, 115])
            cube([6, 20, 20], center = true);
        // Lightening / barrel clearance
        translate([0, 110, 112])
            cube([slide_width - 8, 100, 10], center = true);
        // Muzzle collar seat
        translate([0, 175, 118])
            cylinder(h = 20, d = barrel_od + 1, center = true);
    }
}

module trigger_body() {
    difference() {
        union() {
            // Finger face
            translate([0, 0, 0])
                hull() {
                    translate([0, 0, 0]) cylinder(h = 8, d = 10, center = true);
                    translate([0, 18, -6]) cylinder(h = 6, d = 8, center = true);
                }
            // Pivot ear
            translate([0, -6, 4])
                cube([frame_width - 14, 8, 10], center = true);
        }
        // Pivot hole
        translate([0, -6, 4])
            rotate([0, 90, 0])
                cylinder(h = frame_width, d = trigger_pin_d + clearance, center = true);
        // Pad that presses the tactile switch
        translate([0, 4, -8])
            cube([4, 4, 6], center = true);
    }
}

module magazine_body() {
    difference() {
        rounded_rect([mag_outer_w, mag_outer_d, mag_outer_h], 2);
        // Cell pocket
        translate([0, 0, 8])
            cube([lipo_w + clearance, lipo_h + clearance, lipo_l + 4], center = true);
        // Foam / lead channel to top (toward JST in mag well)
        translate([0, 0, mag_outer_h / 2 - 10])
            cube([10, 8, 30], center = true);
        // Finger grooves (cosmetic, shallow)
        for (z = [-30, -10, 10, 30])
            translate([mag_outer_w / 2 - 1, 0, z])
                rotate([0, 90, 0])
                    cylinder(h = 3, d = 12, center = true);
    }
}

module muzzle_collar_body() {
    difference() {
        cylinder(h = 22, d = barrel_od + 8);
        translate([0, 0, -eps])
            cylinder(h = 24, d = barrel_od + clearance);
        // LED aperture
        translate([0, 0, 12])
            cylinder(h = 12, d = led_d + 1);
        // Flat for glue / set-screw (optional)
        translate([barrel_od / 2 + 2, 0, 11])
            cube([4, 8, 10], center = true);
    }
}

module oled_bezel_body() {
    difference() {
        rounded_rect([oled_pcb_w + 6, oled_pcb_l + 6, 4], 1.5);
        // Window
        translate([0, 0, -eps])
            cube([oled_glass_w + 0.5, oled_glass_l + 0.5, 6], center = true);
        // Clip recesses
        for (sx = [-1, 1])
            translate([sx * (oled_pcb_w / 2 + 1), 0, 1])
                cube([2, 8, 3], center = true);
    }
}

module preview_assembly() {
    color("SteelBlue", 0.85) frame_half(-1);
    color("SteelBlue", 0.55) frame_half(1);
    color("DimGray") translate([0, 0, 2]) slide_body();
    color("Gray") translate([0, 50, 50]) trigger_body();
    color("DarkSlateGray") translate([0, 22, 0]) magazine_body();
    color("DarkOrange") translate([0, 178, 100]) rotate([0, 90, 0]) muzzle_collar_body();
    color("Black") translate([0, 28, 142]) rotate([-oled_tilt_deg, 0, 0]) oled_bezel_body();
}

module export_part() {
    if (part == "preview") preview_assembly();
    else if (part == "frame_left") frame_half(-1);
    else if (part == "frame_right") frame_half(1);
    else if (part == "slide") slide_body();
    else if (part == "trigger") trigger_body();
    else if (part == "magazine") magazine_body();
    else if (part == "muzzle_collar") muzzle_collar_body();
    else if (part == "oled_bezel") oled_bezel_body();
    else echo(str("ERROR: unknown part '", part, "'"));
}

export_part();

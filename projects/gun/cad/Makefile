OPENSCAD ?= openscad
SCAD := gun.scad
PY := export_stl.py
STL_DIR := stl
PARTS := frame_left frame_right slide trigger magazine muzzle_collar oled_bezel

.PHONY: all clean preview help python $(PARTS)

# Prefer OpenSCAD when installed; otherwise the Python fallback meshes.
all:
	@if command -v $(OPENSCAD) >/dev/null 2>&1; then \
		$(MAKE) openscad-all; \
	else \
		echo "OpenSCAD not found — writing approximate STLs with $(PY)"; \
		echo "Install OpenSCAD for the full parametric model: https://openscad.org/"; \
		python3 $(PY); \
	fi

openscad-all: $(addprefix $(STL_DIR)/,$(addsuffix .stl,$(PARTS)))

$(STL_DIR):
	mkdir -p $(STL_DIR)

$(STL_DIR)/%.stl: $(SCAD) | $(STL_DIR)
	$(OPENSCAD) -D 'part="$*"' -o $@ $(SCAD)

python:
	python3 $(PY)

preview:
	@command -v $(OPENSCAD) >/dev/null || { echo "OpenSCAD not found"; exit 1; }
	$(OPENSCAD) $(SCAD)

clean:
	rm -rf $(STL_DIR)

help:
	@echo "make          export STLs (OpenSCAD if present, else Python fallback)"
	@echo "make python   force approximate STLs via $(PY)"
	@echo "make preview  open the assembly in OpenSCAD"
	@echo "make clean    remove $(STL_DIR)/"

use <lib/roundedcube.scad>

backplate_width = 38.65;
backplate_height = 46.40;
backplate_depth = 3.70;
backplate_radius = 6;

module base_round_cube() {
    roundedcube(radius=backplate_radius, size=[backplate_width,backplate_height, 20]);
}

module cut_base_round_cube() {
    scale_radius_compensation = 0.017*2;
    scale([1 + scale_radius_compensation,1 + scale_radius_compensation,1])
    translate([-scale_radius_compensation*14, -scale_radius_compensation*14])
    difference () {
        base_round_cube();
        scale_amount = 0.01;
        translate([-(scale_amount/2), -(scale_amount/2), backplate_depth]) cube([backplate_width + scale_amount, backplate_height + scale_amount, 20]);
    }
}

module roundCubePlate() {
    difference() {
        cut_base_round_cube();
        translate([0, 0, 1]) cut_base_round_cube();
    }
}

module clip(width = 2, height = 3, thickness = 0.5, topThickness = 0.2, topHeight = 0.4, bottomHeight = 1, bottomThickness = 1) {
    difference() {
        union() {
            translate([0, thickness, height - topHeight]) cube([width, topThickness, topHeight]);
            cube([width, thickness, height]);
            cube([width, bottomThickness, bottomHeight]);
        }
        translate([-10, -45, -2.2]) roundCubePlate();
    }
}

clip();

module backplate() {
    roundCubePlate();

    // TODO: placement
    translate([10, 1.5, 2.70]) mirror([0, 1, 0]) clip();
}

backplate();
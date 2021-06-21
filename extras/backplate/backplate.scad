backplate_width = 38.65;
backplate_height = 46.40;
backplate_depth = 3.70;
backplate_radius = 6;
backplate_borderThickness = 0.3;

module base_round_cube() {
    radius = 7;
    v = [backplate_width, backplate_height, backplate_depth + 10];
    vCompensation = [1, 1, 0.5];
    translate(radius * vCompensation)
    minkowski(){
		cube(v - 2 * radius * vCompensation, false);
		scale([radius, radius, radius * 0.5])
            sphere(r=1, $fn=120);
    }
}

module roundCubePlate() {
    difference() {
        difference() {
            color([0.2, 0.5, 0.1]) base_round_cube();
            millimeterToScale = [1 / backplate_width, 1 / backplate_height];
            innerSize = [
                backplate_width - (backplate_borderThickness * 2), 
                backplate_height - (backplate_borderThickness * 2)
            ];
            translate([(backplate_width - innerSize[0]) * 0.5, (backplate_height - innerSize[1]) * 0.5, 1]) 
                scale([innerSize[0] * millimeterToScale[0], innerSize[1] * millimeterToScale[1], 1]) 
                    base_round_cube();
        }
        // To prevent Z fighting
        precisionFix = 0.01;
        translate([precisionFix * -1, precisionFix * -1, backplate_depth]) cube([backplate_width + (precisionFix * 2), backplate_height + (precisionFix * 2), 10]);
    }
}

module clip(width = 2, height = 1.82, thickness = 0.75, topThickness = 0.2, topHeight = 0.4, bottomHeight = 1, bottomThickness = 1) {
    color([0.6, 0.1, 0.5]) render() {
        difference() {
            union() {
                translate([0, thickness, height - topHeight]) cube([width, topThickness, topHeight]);
                cube([width, thickness, height]);
                cube([width, bottomThickness, bottomHeight]);
            }
            translate([-10, (backplate_height * -1) + 1.32, (backplate_depth * -1) + bottomHeight - 0.5]) roundCubePlate();
        }
    }
}
module backplate() {
    roundCubePlate();

    clipWidth = 3.55;
    translate([backplate_width - backplate_borderThickness - 1, 7.69 + clipWidth, 2.70]) rotate([0, 0, -90]) clip(width = clipWidth);
}

backplate();
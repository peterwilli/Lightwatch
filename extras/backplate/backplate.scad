backplate_width = 38.65;
backplate_height = 46.40;
backplate_depth = 3.70;
backplate_radius = 7;
backplate_borderThickness = 0.3;
backplate_borderThicknessBottom = backplate_depth - 2.59;

module base_round_cube() {
    zScale = 0.8;
    v = [backplate_width, backplate_height, backplate_depth + 10];   
    vCompensation = [1, 1, zScale];
    translate(backplate_radius * vCompensation)
    minkowski(){
		cube(v - 2 * backplate_radius * vCompensation, false);
		scale([backplate_radius, backplate_radius, backplate_radius * zScale])
            sphere(r=1, $fn=120);
    }
}

module rounded_cube(size, radius) {
    vCompensation = [1, 1, 1];
    translate(radius * vCompensation)
    minkowski() {
		cube(size - 2 * radius * vCompensation, false);
		scale([radius, radius, radius])
            sphere(r=1, $fn=120);
    }
}

module halfPlate() {
    translate([(backplate_width / 2), (backplate_height / 2), 0]) 
    resize([backplate_width, backplate_height, backplate_depth]) 
    translate([-(backplate_width / 2), -(backplate_height / 2), 0]) 
    difference() {
        color([0.2, 0.5, 0.1]) base_round_cube();
        // To prevent Z fighting
        precisionFix = 0.01;
        translate([precisionFix * -1, precisionFix * -1, backplate_depth]) cube([backplate_width + (precisionFix * 2), backplate_height + (precisionFix * 2), 10]);
    }
}

// !halfPlate();

module roundCubePlate() {
    translate([(backplate_width / 2), (backplate_height / 2), 0]) resize([backplate_width, backplate_height, backplate_depth]) translate([-(backplate_width / 2), -(backplate_height / 2), 0]) difference() {
        difference() {
            color([0.2, 0.5, 0.1]) halfPlate();
            innerSize = [
                backplate_width - (backplate_borderThickness * 2), 
                backplate_height - (backplate_borderThickness * 2),
                backplate_depth - backplate_borderThicknessBottom
            ];
            translate([backplate_borderThickness, backplate_borderThickness, backplate_borderThicknessBottom]) resize([innerSize[0], innerSize[1], innerSize[2]]) halfPlate();
        }
        // To prevent Z fighting
        precisionFix = 0.01;
        translate([precisionFix * -1, precisionFix * -1, backplate_depth]) cube([backplate_width + (precisionFix * 2), backplate_height + (precisionFix * 2), 10]);
    }
}

module clip(width = 2, height = 1.82, thickness = 0.75, topThickness = 0.2, topHeight = 0.4, bottomHeight = 1, bottomThickness = 1) {
    color([0.6, 0.1, 0.5]) {
        difference() {
            union() {
                translate([0, thickness, height - topHeight]) cube([width, topThickness, topHeight]);
                cube([width, thickness, height]);
                cube([width, bottomThickness, bottomHeight]);
            }
            translate([-10, (backplate_height * -1) + 1.65, (backplate_depth * -1) + bottomHeight]) roundCubePlate();
        }
    }
}

module speakerGrill() {
    module speakerGrillHole() {
        rounded_cube([3, 1, 10], 0.2);
    }
    translate([0, 2, 0]) speakerGrillHole();
    translate([0, 4, 0]) speakerGrillHole();
    speakerGrillHole();
}

module backplate() {
    difference() {
        union() {
            roundCubePlate();

            clipWidth = 3.55;
            // Right side
            translate([backplate_width - backplate_borderThickness - 1, 7.90 + clipWidth, 2.70]) rotate([0, 0, -90]) clip(width = clipWidth);
            translate([backplate_width - backplate_borderThickness - 1, backplate_height - 8.90, 2.70]) rotate([0, 0, -90]) clip(width = clipWidth);

            // Left side
            translate([backplate_borderThickness + 1, 8.93, 2.70]) rotate([0, 0, 90]) clip(width = clipWidth);
            translate([backplate_borderThickness + 1, backplate_height - 8.99 - clipWidth, 2.70]) rotate([0, 0, 90]) clip(width = clipWidth);
        }

        translate([2, 8, -5]) speakerGrill();
    }
}

backplate();
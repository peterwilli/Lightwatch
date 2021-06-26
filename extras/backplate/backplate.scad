backplate_width = 38.65;
backplate_height = 46.40;
backplate_depth = 3.80;
backplate_radius = 7;
backplate_borderThickness = 0.5;
backplate_borderThicknessBottom = 1;

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

module clipIntersector() {
    union() {
        halfPlate();
        translate([0, 0, backplate_depth]) cube([backplate_width, backplate_height, 10]);
    }
}

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

module clip(width = 2, height = 1.82, thickness = 0.95, topThickness = 0.2, topHeight = 0.4, bottomHeight = backplate_depth, bottomThickness = 2) {
    color([0.6, 0.1, 0.5]) {
        intersection() {
            translate([0, 0, bottomHeight]) union() {
                translate([0, thickness, height - (topThickness * 0.5)]) scale([1, 1, 0.5]) rotate([90, 0, 90]) cylinder(r=topThickness, h=width, center=false, $fn=50);
                cube([width, thickness, height]);
                translate([0, 0, bottomHeight * -1]) cube([width, bottomThickness, bottomHeight]);
            }
            translate([-10, (backplate_height * -1) + bottomThickness, 0]) clipIntersector();
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
            clipHeight = 1.82;
            clipWidth = 3.55;
            clipMaxThickness = 1;
            borderOffset = 1.40;
            // Left side
            translate([clipMaxThickness + borderOffset, 8.93, 0]) rotate([0, 0, 90]) clip(width = clipWidth);
            translate([clipMaxThickness + borderOffset, backplate_height - 8.99 - clipWidth, 0]) rotate([0, 0, 90]) clip(width = clipWidth);
            
            // Right side
            translate([backplate_width - clipMaxThickness - borderOffset, 7.90 + clipWidth, 0]) rotate([0, 0, -90]) clip(width = clipWidth);
            translate([backplate_width - clipMaxThickness - borderOffset, backplate_height - 8.90, 0]) rotate([0, 0, -90]) clip(width = clipWidth);
        }

        translate([4, 8, -5]) speakerGrill();
    }
}

backplate();
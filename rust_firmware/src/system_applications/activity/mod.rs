mod detectors;
use crate::alloc::string::ToString;
use crate::c_bindings::*;
use crate::gui::*;
use crate::loop_time::loop_time;

use crate::system_applications::system_application::*;
use crate::SerialLogger;
use alloc::format;
use alloc::vec;


use detectors::Detector;
use detectors::SkippingRopeDetector;
use std::prelude::v1::*;

pub struct ActivityApplication {
    gui_renderer: GUIRenderer,
    current_detector: SkippingRopeDetector,
    last_accel_time: u32,
}

impl SystemApplication for ActivityApplication {
    fn new() -> Self {
        return {
            ActivityApplication {
                gui_renderer: GUIRenderer::new(),
                current_detector: SkippingRopeDetector::new(),
                last_accel_time: 0,
            }
        };
    }

    fn get_info(&self) -> SystemApplicationInfo {
        return SystemApplicationInfo {
            id: "lightwatch.activity".to_string(),
            name: "Activity".to_string(),
            description: "Fitness activity application".to_string(),
            extras: vec![Extra::BackgroundLoop],
        };
    }

    fn init(&mut self) {
        unsafe {
            enableAccelerometer();
            fillScreen(2821);
            let mut label = Box::new(Label::new(10, 10, 100, 100));
            label.font_size = 2;
            label.text = Some("Jump count".to_string());
            self.gui_renderer.elements.push(label);
        }
    }

    fn r#loop(&mut self) {
        unsafe {
            if loop_time.millis > self.last_accel_time {
                let mut accel = Accel { x: 0, y: 0, z: 0 };
                let _x = readAccelerometer(&mut accel);
                SerialLogger::println(format!("accel: {}x{}x{}", accel.x, accel.y, accel.z));
                self.current_detector.push_accel(&accel);
                let mut counter_label: &mut Label = self.gui_renderer.elements[0]
                    .as_any()
                    .downcast_mut::<Label>()
                    .expect("Wasn't a label!");
                let old_text = counter_label.text.clone();
                counter_label.text =
                    Some(format!("Jump count: {}", self.current_detector.jump_count));
                if old_text.as_deref() != counter_label.text.as_deref() {
                    self.gui_renderer.needs_redraw = true;
                }
                self.last_accel_time = loop_time.millis + 100;
            }
        }
        if self.gui_renderer.will_redraw() {
            unsafe {
                fillScreen(0);
            }
        }
        self.gui_renderer.r#loop();
    }
}

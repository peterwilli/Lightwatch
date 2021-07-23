use crate::alloc::string::ToString;
use crate::c_bindings::*;
use crate::non_official_c_bindings::*;
use crate::gui::*;

use crate::system_applications::system_application::*;
use crate::system_applications::LucidDreamingApplication;
use crate::system_applications::ActivityApplication;

use alloc::vec;

use no_std_compat::sync::Mutex;
use std::prelude::v1::*;

pub struct HomeScreenApplication {
    gui_renderer: GUIRenderer,
    rtc_date: RTCDate,
    last_render_time: u32
}

struct HomeScreenState {
    current_application: Option<Box<dyn SystemApplication>>,
}

unsafe impl Send for HomeScreenState {}
unsafe impl Sync for HomeScreenState {}

lazy_static! {
    static ref HOME_SCREEN_STATE: Mutex<HomeScreenState> = Mutex::new(HomeScreenState {
        current_application: None
    });
}

fn launch_app(app: Box<dyn SystemApplication>) {
    let mut home_screen_state = HOME_SCREEN_STATE.lock();
    if home_screen_state.current_application.is_none() {
        home_screen_state.current_application = Some(app);
        home_screen_state
            .current_application
            .as_mut()
            .unwrap()
            .init();
    }
}

impl HomeScreenApplication {
    fn update_time(&mut self) {
        unsafe {
            rtc_getDateTime(&mut self.rtc_date);
        }
        let mut time_label: &mut Label = self.gui_renderer.elements[3]
                .as_any()
                .downcast_mut::<Label>()
                .expect("Wasn't a label!");
        time_label.text = Some(format!("{} {}", self.rtc_date.hour, self.rtc_date.minute));
        self.gui_renderer.needs_redraw = true;
    }

    fn update_date(&mut self) {
        unsafe {
            rtc_getDateTime(&mut self.rtc_date);
        }
        let mut date_label: &mut Label = self.gui_renderer.elements[4]
                .as_any()
                .downcast_mut::<Label>()
                .expect("Wasn't a label!");
        date_label.text = Some(format!("{} {}", self.rtc_date.day, self.rtc_date.month));
        self.gui_renderer.needs_redraw = true;
    }
}

impl SystemApplication for HomeScreenApplication {
    fn new() -> Self {
        return {
            HomeScreenApplication {
                gui_renderer: GUIRenderer::new(),
                rtc_date: RTCDate {
                    year: 0,
                    month: 0,
                    day: 0,
                    hour: 0,
                    minute: 0,
                    second: 0
                },
                last_render_time: 0
            }
        };
    }

    fn get_info(&self) -> SystemApplicationInfo {
        return SystemApplicationInfo {
            id: "lightwatch.home".to_string(),
            name: "Home".to_string(),
            description: "Home screen".to_string(),
            extras: vec![Extra::BackgroundLoop],
        };
    }

    fn init(&mut self) {
        if unsafe { getRTCDataAtIndex(0) } > 0 {
            launch_app(Box::new(LucidDreamingApplication::new()));
        } else {
            unsafe {
                setBrightness(50);
            }
            unsafe {
                fillScreen(0);
                setTextColor(400);
            }
            let mut label = Box::new(Label::new(10, 10, 100, 100));
            label.text = Some("Hello Loves".to_string());
            self.gui_renderer.elements.push(label);

            let mut button = Box::new(Button::new(10, 30, 100, 100));
            button.text = Some("Activity".to_string());
            button.on_tap = Some(Box::new(|| {
                launch_app(Box::new(ActivityApplication::new()));
            }));
            self.gui_renderer.elements.push(button);

            let mut button = Box::new(Button::new(10, 90, 100, 100));
            button.text = Some("Lucid dreaming".to_string());
            button.on_tap = Some(Box::new(|| {
                launch_app(Box::new(LucidDreamingApplication::new()));
            }));
            self.gui_renderer.elements.push(button);

            let mut label = Box::new(Label::new(10, 120, 100, 100));
            self.gui_renderer.elements.push(label);

            let mut label = Box::new(Label::new(10, 150, 100, 100));
            self.gui_renderer.elements.push(label);

            self.update_time();
            self.update_date();
        }
    }

    fn r#loop(&mut self) {
        let cur_render_time = unsafe { millis() };
        if cur_render_time - self.last_render_time > 10000 {
            self.update_time();
            self.update_date();
            self.last_render_time = cur_render_time;
        }
        let mut home_screen_state = HOME_SCREEN_STATE.lock();
        if home_screen_state.current_application.is_none() {
            drop(home_screen_state);
            if self.gui_renderer.will_redraw() {
                unsafe {
                    fillScreen(0);
                }
            }
            self.gui_renderer.r#loop();
        } else {
            let current_app = home_screen_state.current_application.as_mut().unwrap();
            current_app.r#loop();
        }
        unsafe {
            delay(100);
        }
    }
}

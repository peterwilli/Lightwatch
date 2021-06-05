use crate::alloc::string::ToString;
use crate::c_bindings::*;
use crate::gui::*;
use crate::system_applications::system_application::*;
use crate::system_applications::ActivityApplication;
use crate::system_applications::LucidDreamingApplication;
use crate::SerialLogger;
use alloc::sync::Arc;
use alloc::vec;
use cstr_core::CString;
use no_std_compat::sync::Mutex;
use std::prelude::v1::*;

pub struct HomeScreenApplication {
    gui_renderer: GUIRenderer,
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

fn launch_app(app: Box<SystemApplication>) {
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

impl SystemApplication for HomeScreenApplication {
    fn new() -> Self {
        return {
            HomeScreenApplication {
                gui_renderer: GUIRenderer::new(),
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
        unsafe {
            fillScreen(1929);
            setTextColor(400);
        }
        let mut label = Box::new(Label::new(10, 10, 100, 100));
        label.text = Some("Hello Loves".to_string());
        let mut button = Box::new(Button::new(10, 30, 100, 100));
        button.text = Some("Button".to_string());
        button.on_tap = Some(Box::new(|| {
            launch_app(Box::new(LucidDreamingApplication::new()));
        }));
        self.gui_renderer.elements.push(label);
        self.gui_renderer.elements.push(button);
    }

    fn r#loop(&mut self) {
        let mut home_screen_state = HOME_SCREEN_STATE.lock();
        if home_screen_state.current_application.is_none() {
            drop(home_screen_state);
            self.gui_renderer.r#loop();
        } else {
            let current_app = home_screen_state.current_application.as_mut().unwrap();
            current_app.r#loop();
        }
    }
}

pub struct ButtonInput {
    pub is_pressed: bool,
}

pub static mut button_input: ButtonInput = ButtonInput { is_pressed: false };

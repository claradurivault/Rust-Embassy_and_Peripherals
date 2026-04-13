use embassy_stm32::gpio::{Input, AnyPin, Pull};
use embassy_stm32::Peri;
use {defmt_rtt as _, panic_probe as _};

use crate::drivers::button::Button;

pub struct GamepadState {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub select: bool,
}

impl GamepadState{
    pub fn display(&self) -> &'static str {
        match (self.top, self.bottom, self.left, self.right, self.select) {
            (true, false, false, false, false) => "Top",
            (false, true, false, false, false) => "Bottom",
            (false, false, true, false, false) => "Left",
            (false, false, false, true, false) => "Right",
            (false, false, false, false, true) => "Select",
            _ => "Multiple or no buttons pressed",
        }
    }
}
pub struct Gamepad {
    btn_top : Button,
    btn_bottom : Button,
    btn_left : Button,
    btn_right : Button,
    btn_select : Button,
}

impl Gamepad{
    pub fn new(btn_top: Peri<'static, AnyPin>, btn_bottom: Peri<'static, AnyPin>, btn_left: Peri<'static, AnyPin>, btn_right: Peri<'static, AnyPin>, btn_select: Peri<'static, AnyPin>) -> Self {
        let btn_top = Button { button: Input::new(btn_top, Pull::Down) };
        let btn_bottom = Button { button: Input::new(btn_bottom, Pull::Down) };
        let btn_left = Button { button: Input::new(btn_left, Pull::Down) };
        let btn_right = Button { button: Input::new(btn_right, Pull::Down) };
        let btn_select = Button { button: Input::new(btn_select, Pull::Down) };
        Self {btn_top, btn_bottom, btn_left, btn_right, btn_select}
    }

    pub fn poll(&self) -> GamepadState {
        GamepadState {
            top: self.btn_top.is_pressed(),
            bottom: self.btn_bottom.is_pressed(),
            left: self.btn_left.is_pressed(),
            right: self.btn_right.is_pressed(),
            select: self.btn_select.is_pressed(),
        }
    }
}
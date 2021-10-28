use wpilib::ds::{JoystickAxis, JoystickPort};

pub fn PORT_1() -> JoystickPort {
    JoystickPort::new(0).unwrap()
}

pub fn LEFT_X() -> JoystickAxis {
    JoystickAxis::new(0).unwrap()
}
pub fn LEFT_Y() -> JoystickAxis {
    JoystickAxis::new(1).unwrap()
}
pub fn RIGHT_TRIGGER() -> JoystickAxis {
    JoystickAxis::new(3).unwrap()
}
// pub fn LEFT_TRIGGER = 2;
// pub fn RIGHT_X = 4;
// pub fn RIGHT_Y = 5;

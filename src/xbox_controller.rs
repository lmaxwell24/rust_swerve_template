use wpilib::ds::{JoystickAxis, JoystickPort};

pub const PORT_1: JoystickPort = JoystickPort::new(0).unwrap();

pub const LEFT_X: JoystickAxis = JoystickAxis::new(0).unwrap();
pub const LEFT_Y: JoystickAxis = JoystickAxis::new(1).unwrap();
pub const RIGHT_TRIGGER: JoystickAxis = JoystickAxis::new(3).unwrap();
// pub const LEFT_TRIGGER = 2;
// pub const RIGHT_X = 4;
// pub const RIGHT_Y = 5;

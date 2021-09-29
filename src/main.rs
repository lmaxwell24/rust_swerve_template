use wpilib;
mod robot;
mod xbox_controller;
use ctre::motor_control::{BaseMotorController, ControlMode, DemandType, TalonSRX};

struct Robot<'a> {
    left_motor: TalonSRX,
    right_motor: TalonSRX,
    ds: wpilib::ds::DriverStation<'a>,
}

impl robot::IterativeRobot for Robot<'_> {
    fn new<'a>(ds: &'a wpilib::ds::DriverStation) -> Robot<'a> {
        Robot {
            left_motor: TalonSRX::new(1),
            right_motor: TalonSRX::new(2),
            ds: ds.clone(),
        }
    }
    fn teleop_init(&mut self) {
        println!("start teleop");
    }
    fn teleop_periodic(&mut self) {
        let fw = self
            .ds
            .stick_axis(xbox_controller::PORT_1, xbox_controller::LEFT_Y)
            .unwrap() as f64;
        let turn = self
            .ds
            .stick_axis(xbox_controller::PORT_1, xbox_controller::LEFT_X)
            .unwrap() as f64;
        self.left_motor.set(
            ControlMode::PercentOutput,
            fw,
            DemandType::ArbitraryFeedForward,
            turn,
        );
        self.right_motor.set(
            ControlMode::PercentOutput,
            fw,
            DemandType::ArbitraryFeedForward,
            -turn,
        ); //simple arcade drive
    }
}

fn main() {
    robot::start_timed::<Robot>();
}

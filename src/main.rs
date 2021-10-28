use wpilib::{self, pwm::PwmSpeedController};
mod constants;
mod robot;
mod xbox_controller;
use ctre::motor_control::{BaseMotorController, ControlMode, DemandType, TalonSRX};

struct CameraServo {
    up_down: wpilib::pwm::PwmSpeedController,
    left_right: wpilib::pwm::PwmSpeedController,
}

struct Robot {
    left_motor: TalonSRX,
    right_motor: TalonSRX,
    cam: CameraServo,
}

impl robot::IterativeRobot for Robot {
    fn new(ds: &wpilib::ds::DriverStation) -> Robot {
        Robot {
            left_motor: TalonSRX::new(1),
            right_motor: TalonSRX::new(2),
            cam: CameraServo {
                up_down: PwmSpeedController::new(0).unwrap(),
                left_right: PwmSpeedController::new(1).unwrap(),
            },
        }
    }
    fn teleop_init(&mut self) {
        println!("start teleop");
    }
    fn teleop_periodic(&mut self, ds: &wpilib::ds::DriverStation) {
        let fw = ds
            .stick_axis(xbox_controller::PORT_1(), xbox_controller::LEFT_Y())
            .unwrap() as f64;
        let turn = ds
            .stick_axis(xbox_controller::PORT_1(), xbox_controller::LEFT_X())
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

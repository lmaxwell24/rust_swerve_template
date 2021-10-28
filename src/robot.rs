use std::time;
use wpilib::{
    ds::{DriverStation, RobotState},
    RobotBase,
};

/// Implements a specific type of robot program framework, for
/// `start_iterative` and `start_timed`.
///
/// The init methods are called whenever the appropriate mode is entered.
///
/// The periodic functions are called for the appropriate mode on an interval.
pub trait IterativeRobot {
    /// Robot constructor.
    ///
    /// Create your resources here and return an instance of your robot.
    fn new(ds: &DriverStation) -> Self;

    fn disabled_init(&mut self) {
        println!("Default disabled_init method... Override me!");
    }
    fn autonomous_init(&mut self) {
        println!("Default autonomous_init method... Override me!");
    }
    fn teleop_init(&mut self) {
        println!("Default teleop_init method... Override me!");
    }
    fn test_init(&mut self) {
        println!("Default test_init method... Override me!");
    }

    /// Periodic code for all modes should go here.
    fn robot_periodic(&mut self) {}

    fn disabled_periodic(&mut self) {}
    fn autonomous_periodic(&mut self) {}
    fn teleop_periodic(&mut self, ds: &DriverStation) {}
    fn test_periodic(&mut self) {}
}

fn loop_func<T: IterativeRobot>(
    robot: &mut T,
    last_mode: Option<RobotState>,
    cur_mode: RobotState,
    ds: &DriverStation,
) {
    // Check for state transitions
    if last_mode != Some(cur_mode) {
        match cur_mode {
            RobotState::Autonomous => robot.autonomous_init(),
            RobotState::Teleop => robot.teleop_init(),
            RobotState::Test => robot.test_init(),
            _ => robot.disabled_init(),
        }
    }

    // Call the appropriate periodic function
    match cur_mode {
        RobotState::Autonomous => robot.autonomous_periodic(),
        RobotState::Teleop => robot.teleop_periodic(ds),
        RobotState::Test => robot.test_periodic(),
        _ => robot.disabled_periodic(),
    }

    robot.robot_periodic()
}

/// Start the main robot loop for an IterativeRobot.
/// The periodic methods are called each time a new packet
/// received from the driver station.
///
/// It is recommended to use `start_timed` instead,
/// which guarantees a more regular period of execution.
pub fn start_iterative<T: IterativeRobot>() -> ! {
    let base = RobotBase::new().expect("Could not initialize HAL");
    let ds = base.make_ds();

    println!("\n********** Robot program starting **********\n");

    let mut robot = T::new(&ds);
    let mut last_mode: Option<RobotState> = None;

    // Tell the DS that the robot is ready to be enabled

    loop {
        ds.wait_for_data();

        let cur_mode = ds.robot_state();
        loop_func(&mut robot, last_mode, cur_mode, &ds);
        last_mode = Some(cur_mode);
    }
}

/// Start the main robot loop for an IterativeRobot.
/// The periodic methods are called every 20 milliseconds.
///
/// If you wish to have your main loop run at a different rate,
/// use `start_timed_with_period`.
pub fn start_timed<T: IterativeRobot>() {
    start_timed_with_period::<T>(time::Duration::from_millis(20))
}

/// Start the main robot loop for an IterativeRobot.
/// The periodic methods are called on a regular interval specified by `period`.
pub fn start_timed_with_period<T: IterativeRobot>(period: time::Duration) {
    let base = RobotBase::new().expect("Could not initialize HAL");
    let ds = base.make_ds();

    println!("\n********** Robot program starting **********\n");

    let mut robot = T::new(&ds);
    let mut last_mode: Option<RobotState> = None;

    // Tell the DS that the robot is ready to be enabled

    loop {
        let cur_mode = ds.robot_state();
        loop_func(&mut robot, last_mode, cur_mode, &ds);
        last_mode = Some(cur_mode);
        ::std::thread::sleep(period);
    }
}

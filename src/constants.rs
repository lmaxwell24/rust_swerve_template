pub mod SimPorts {
    pub mod Front {
        pub const LEFT_DRIVE: i32 = 10;
        pub const LEFT_STEER: i32 = 11;
        pub const RIGHT_DRIVE: i32 = 12;
        pub const RIGHT_STEER: i32 = 13;
    }
    pub mod Back {
        pub const LEFT_DRIVE: i32 = 14;
        pub const LEFT_STEER: i32 = 15;
        pub const RIGHT_DRIVE: i32 = 16;
        pub const RIGHT_STEER: i32 = 17;
    }
    pub mod Motor {
        pub const DRIVE_P_GAIN: f32 = 0.01;
    }
}

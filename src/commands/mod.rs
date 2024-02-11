pub mod serde;

#[derive(Debug)]
pub enum Module {
    Ballast,
    Light,
    Propulsion,
}

#[derive(Debug, Copy, Clone)]
pub enum BallastCommand {
    Idle,
    Intake,
    Discharge,
}

#[derive(Copy, Clone, Debug)]
pub enum PropulsionCommand {
    SetThrust(DirectionVector),
}
#[derive(Copy, Clone, Debug)]
pub struct DirectionVector {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug)]
pub enum LightCommand {
    On,
    Off,
    Blink,
}

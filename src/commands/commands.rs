use super::serde;
use crate::definitions::DirectionVector;
use std::sync::Arc;

#[derive(Debug, Copy, Clone)]
pub enum BallastCommand {
    Idle,
    Intake,
    Discharge,
}

// One byte: 0: stop, 1: intake mode, 2: discharge mode
// [ [] [][][][][][][][][][][] ]
impl serde::Serde for BallastCommand {
    fn deserialize(
        command_payload: &[u8]
    ) -> Result<Arc<Self>, ()> {
        match command_payload[0] {
            0 => Ok(Arc::new(BallastCommand::Idle)),
            1 => Ok(Arc::new(BallastCommand::Intake)),
            2 => Ok(Arc::new(BallastCommand::Discharge)),
            _ => Err(())
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub enum PropulsionCommand {
    SetThrust(DirectionVector),
}

//   x: f32   y: f32   unused
// [ [][][][] [][][][] [][][][] ]
impl serde::Serde for PropulsionCommand {
    fn deserialize(
        command_payload: &[u8]
    ) -> Result<Arc<Self>, ()> {
        let mut x_component: [u8; 4] = [0; 4];
        let mut y_component: [u8; 4] = [0; 4];

        x_component[0] = command_payload[0];
        x_component[1] = command_payload[1];
        x_component[2] = command_payload[2];
        x_component[3] = command_payload[3];

        y_component[0] = command_payload[4];
        y_component[1] = command_payload[5];
        y_component[2] = command_payload[6];
        y_component[3] = command_payload[7];

        let x_component: f32 = f32::from_le_bytes(x_component);
        let y_component: f32 = f32::from_le_bytes(y_component);

        Ok(Arc::new(PropulsionCommand::SetThrust(DirectionVector {
            x: x_component,
            y: y_component,
        })))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LightCommand {
    On,
    Off,
    Blink,
}

// One byte: 0: Off, 1: On, 2: Blink
// [ [] [][][][][][][][][][][] ]
impl serde::Serde for LightCommand {
    fn deserialize(
        command_payload: &[u8]
    ) -> Result<Arc<Self>, ()> {
        match command_payload[0] {
            0 => Ok(Arc::new(LightCommand::Off)),
            1 => Ok(Arc::new(LightCommand::On)),
            2 => Ok(Arc::new(LightCommand::Blink)),
            _ => Err(())
        }
    }
}

use std::collections::HashMap;
use once_cell::sync::Lazy;
use super::*;
use std::sync::Arc;

/*
 *  [(start_byte)(module_id_byte)(payload..)(TODO: checksum)(end_byte)]
 */
pub const COMMAND_BUFFER_SIZE: usize = 16;
#[allow(dead_code)]
pub const COMMAND_PAYLOAD_SIZE: usize = COMMAND_BUFFER_SIZE - 3;
const COMMAND_BUFFER_START_BYTE: u8 = 0xA;
const COMMAND_BUFFER_END_BYTE: u8 = 0xF;
const BALLAST_ID: u8 = 0x0;
const LIGHT_ID: u8 = 0x2;
const PROP_ID: u8 = 0x1;

pub const MODULE_IDS: Lazy<HashMap<u8, Module>> = Lazy::new(|| {
    let mut map: HashMap<u8, Module> = HashMap::new();

    map.insert(BALLAST_ID, Module::Ballast);
    map.insert(LIGHT_ID, Module::Light);
    map.insert(PROP_ID, Module::Propulsion);

    map
});

pub trait Serde {
    fn deserialize(
        command_payload: &[u8]
    ) -> Result<Arc<Self>, ()>;

    fn serialize(&self) -> [u8; COMMAND_BUFFER_SIZE];
}

pub fn validate_command_structure(
    command_buffer: &[u8; COMMAND_BUFFER_SIZE]
) -> bool {
    let start: u8 = command_buffer[0];
    let end: u8 = command_buffer[COMMAND_BUFFER_SIZE - 1];
    let module_id: u8 = command_buffer[1];

    if start != COMMAND_BUFFER_START_BYTE
    || end != COMMAND_BUFFER_END_BYTE {
        eprintln!(
            "Start ({}|{}) or end ({}|{}) byte is not correct.",
            start,
            COMMAND_BUFFER_START_BYTE,
            end,
            COMMAND_BUFFER_END_BYTE
        );
        return false;
    }

    if !MODULE_IDS.contains_key(&module_id) {
        eprintln!("Invalid module ID: {}\nValid keys: {:?}",
                  module_id,
                  MODULE_IDS.keys());
        return false;
    }

    // TODO: checksum

    true
}

fn create_command_buffer_template() -> [u8; COMMAND_BUFFER_SIZE] {
    let mut buf: [u8; COMMAND_BUFFER_SIZE] = [0; COMMAND_BUFFER_SIZE];

    buf[0] = COMMAND_BUFFER_START_BYTE;
    buf[COMMAND_BUFFER_SIZE - 1] = COMMAND_BUFFER_END_BYTE;

    buf
}

//
// One byte: 0: stop, 1: intake mode, 2: discharge mode
// [ [] [][][][][][][][][][][] ]
impl Serde for BallastCommand {
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

    fn serialize(&self) -> [u8; COMMAND_BUFFER_SIZE] {
        let mut buf = create_command_buffer_template();
        buf[1] = BALLAST_ID;

        let ballast_state = match self {
            Self::Idle => 0,
            Self::Intake => 1,
            Self::Discharge => 2,
        };

        buf[2] = ballast_state;

        buf
    }
}

//   x: f32   y: f32   unused
// [ [][][][] [][][][] [][][][] ]
impl Serde for PropulsionCommand {
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

    fn serialize(&self) -> [u8; COMMAND_BUFFER_SIZE] {
        let mut buf = create_command_buffer_template();

        let vec = match self {
            Self::SetThrust(thrust_vec) => thrust_vec,
        };

        let x_bytes = vec.x.to_le_bytes();
        let y_bytes = vec.y.to_le_bytes();

        buf[1] = PROP_ID;

        buf[2] = x_bytes[0];
        buf[3] = x_bytes[1];
        buf[4] = x_bytes[2];
        buf[5] = x_bytes[3];

        buf[6] = y_bytes[0];
        buf[7] = y_bytes[1];
        buf[8] = y_bytes[2];
        buf[9] = y_bytes[3];

        buf
    }
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

    fn serialize(&self) -> [u8; COMMAND_BUFFER_SIZE] {
        todo!()

    }
}

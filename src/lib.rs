mod parse;

pub use parse::{parse_message, FractalMessage };

type MidiMessage = Vec<u32>;

#[derive(PartialEq, Debug)]
pub enum FractalModel {
    Standard,
    Ultra,
    MFC101,
    II,
    MFC101MK3,
    FX8,
    IIXL,
    IIXLPlus,
    AX8,
    FX8MK2,
    III,
}

fn model_code(model: FractalModel) -> u32 {
    match model {
        FractalModel::Standard => 0x00,
        FractalModel::Ultra => 0x01,
        FractalModel::MFC101 => 0x02,
        FractalModel::II => 0x03,
        FractalModel::MFC101MK3 => 0x04,
        FractalModel::FX8 => 0x05,
        FractalModel::IIXL => 0x06,
        FractalModel::IIXLPlus => 0x07,
        FractalModel::AX8 => 0x08,
        FractalModel::FX8MK2 => 0x0A,
        FractalModel::III => 0x10,
    }
}

pub fn checksum(msg: MidiMessage) -> u32 {
    let xord = msg
        .iter()
        .take(msg.len() - 1)
        .fold(None, |acc: Option<u32>, x| match acc {
            Some(y) => Some(y ^ x),
            None => Some(*x),
        })
        .unwrap();
    0x7F & xord
}

pub fn with_checksum(msg: MidiMessage) -> MidiMessage {
    let term = msg.iter().last().unwrap();
    let msg_checksum = checksum(msg.clone());
    let msg_without_term: MidiMessage = msg
        .clone()
        .into_iter()
        .take(msg.len() - 1)
        .collect::<Vec<u32>>();
    [msg_without_term, vec![msg_checksum, *term]].concat()
}

fn wrap_msg(msg: MidiMessage) -> MidiMessage {
    let header = vec![0xF0, 0x00, 0x01, 0x74];
    with_checksum([header, msg, vec![0xF7]].concat())
}

pub fn get_preset_number(model: FractalModel) -> MidiMessage {
    wrap_msg(vec![model_code(model), 0x14])
}

pub fn get_current_preset_name(model: FractalModel) -> MidiMessage {
    wrap_msg(vec![model_code(model), 0x0F])
}

fn encode_preset_number(n: u32) -> (u32, u32) {
    (n >> 7, n & 0x7F)
}

pub fn set_preset_number(model: FractalModel, n: u32) -> MidiMessage {
    let (a, b) = encode_preset_number(n);
    wrap_msg(vec![model_code(model), 0x3C, a, b])
}

pub fn set_current_preset_name(model: FractalModel, name: &str) -> MidiMessage {
    let namesci: Vec<u32> = name.chars()
        .filter(|c| c.is_ascii())
        .map(|c| c as u32).collect();
    let pad: Vec<u32> = (0..(32 - namesci.len())).map(|_| 32).collect();
    wrap_msg([vec![model_code(model), 0x09], namesci, pad].concat())
}

pub fn get_firmware_version(model: FractalModel) -> MidiMessage {
    wrap_msg(vec![model_code(model), 0x08])
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_checksum() {
        assert_eq!(
            0x09,
            checksum(vec![0xF0, 0x00, 0x01, 0x74, 0x03, 0x0F, 0xF7])
        );
    }

    #[test]
    fn test_with_checksum() {
        assert_eq!(
            vec![0xF0, 0x00, 0x01, 0x74, 0x03, 0x0F, 0x09, 0xF7],
            with_checksum(vec![0xF0, 0x00, 0x01, 0x74, 0x03, 0x0F, 0xF7])
        );
        assert_eq!(
            vec![0xF0, 0x00, 0x01, 0x74, 0x03, 0x14, 18, 0xF7],
            with_checksum(vec![0xF0, 0x00, 0x01, 0x74, 0x03, 0x14, 0xF7])
        );
    }

    #[test]
    fn test_get_preset_number() {
        assert_eq!(
            vec![
                0xF0,
                0x00,
                0x01,
                0x74,
                model_code(FractalModel::II),
                0x14,
                18,
                0xF7
            ],
            get_preset_number(FractalModel::II)
        );
    }

    #[test]
    fn test_set_preset_number() {
        assert_eq!(
            vec![
                0xF0,
                0x00,
                0x01,
                0x74,
                model_code(FractalModel::II),
                0x3C,
                0,
                127,
                69,
                0xF7
            ],
            set_preset_number(FractalModel::II, 127)
        );
        assert_eq!(
            vec![
                0xF0,
                0x00,
                0x01,
                0x74,
                model_code(FractalModel::II),
                0x3C,
                1,
                0,
                59,
                0xF7
            ],
            set_preset_number(FractalModel::II, 128)
        );
    }

    #[test]
    fn test_parse_message_preset_number() {
        assert_eq!(
            FractalMessage::CurrentPresetNumber(235),
            parse_message(vec![240, 0, 1, 116, 3, 20, 1, 107, 120, 247])
        );
        assert_eq!(
            FractalMessage::CurrentPresetNumber(236),
            parse_message(vec![240, 0, 1, 116, 3, 20, 1, 108, 121, 247])
        );
    }

    #[test]
    fn test_get_current_preset_name() {
        assert_eq!(
            vec![
                0xF0,
                0x00,
                0x01,
                0x74,
                model_code(FractalModel::II),
                0x0F,
                9,
                0xF7
            ],
            get_current_preset_name(FractalModel::II)
        );
    }

    #[test]
    fn test_set_current_preset_name() {
        assert_eq!(
            vec![0xF0 ,0x00 ,0x01 ,0x74 ,model_code(FractalModel::II) ,0x09 ,0x43 ,0x68 ,0x61 ,0x6E ,0x67 ,0x65 ,0x64 ,0x21 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x20 ,0x6C ,0xF7],
            set_current_preset_name(FractalModel::II, "Changed!")
        );
        assert_eq!(
            set_current_preset_name(FractalModel::II, "O Praise The Name (Anstasis)"),
            set_current_preset_name(FractalModel::II, "O Praise The Name (Anástasis)")
        );
    }

    #[test]
    fn test_parse_current_preset_name() {
        assert_eq!(
            parse_message(vec![240 ,0 ,1 ,116 ,3 ,15 ,66 ,83 ,32 ,65 ,67 ,50 ,48 ,32 ,66 ,97 ,115 ,101 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,32 ,0 ,13 ,247]),
            FractalMessage::CurrentPresetName("BS AC20 Base".to_string())
        );
    }

    #[test]
    fn test_get_firmware_version() {
        assert_eq!(
            vec![0xF0 ,0x00 ,0x01 ,0x74 ,model_code(FractalModel::II) ,0x08 ,14 ,0xF7],
            get_firmware_version(FractalModel::II)
        );
    }

    #[test]
    fn test_parse_firmware_version() {
        assert_eq!(
            parse_message(vec![240 ,0 ,1 ,116 ,3 ,8 ,0x08 ,0x02 ,0 ,0 ,0 ,0 ,0,247]),
            FractalMessage::FirmwareVersion { major: 8, minor: 2 }
        );
    }
}

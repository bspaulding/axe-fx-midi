type MidiMessage = Vec<usize>;

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

fn model_code(model: FractalModel) -> usize {
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

pub fn checksum(msg: MidiMessage) -> usize {
    let xord = msg
        .iter()
        .take(msg.len() - 1)
        .fold(None, |acc: Option<usize>, x| match acc {
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
        .collect::<Vec<usize>>();
    [msg_without_term, vec![msg_checksum, *term]].concat()
}

fn wrap_msg(msg: MidiMessage) -> MidiMessage {
    let header = vec![0xF0, 0x00, 0x01, 0x74];
    with_checksum([header, msg, vec![0xF7]].concat())
}

pub fn get_preset_number(model: FractalModel) -> MidiMessage {
    wrap_msg(vec![model_code(model), 0x14])
}

fn encode_preset_number(n: usize) -> (usize, usize) {
    (n >> 7, n & 0x7F)
}

pub fn set_preset_number(model: FractalModel, n: usize) -> MidiMessage {
    let (a, b) = encode_preset_number(n);
    wrap_msg(vec![model_code(model), 0x3C, a, b])
}

fn decode_preset_number(lsb: usize, rsb: usize) -> usize {
    ((lsb & 0x7F) << 7) | rsb
}

#[derive(PartialEq, Debug)]
pub enum FractalMessage {
    Unknown(MidiMessage),
    CurrentPresetNumber(usize),
}

pub fn parse_message(msg: MidiMessage) -> FractalMessage {
    let function_id = msg.iter().nth(5).unwrap();
    match function_id {
        20 => FractalMessage::CurrentPresetNumber(decode_preset_number(
            *msg.iter().nth(6).unwrap(),
            *msg.iter().nth(7).unwrap(),
        )),
        _ => FractalMessage::Unknown(msg),
    }
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
}

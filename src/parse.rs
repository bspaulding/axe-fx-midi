use crate::MidiMessage;

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

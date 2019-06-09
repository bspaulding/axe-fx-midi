use crate::MidiMessage;

fn decode_preset_number(lsb: u32, rsb: u32) -> u32 {
    ((lsb & 0x7F) << 7) | rsb
}

fn decode_preset_name(msg: Vec<u32>) -> String {
    msg.iter()
        .take(32)
        .filter(|x| *x > &0)
        .map(|x| *x as u8 as char)
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[derive(PartialEq, Debug)]
pub enum FractalMessage {
    Unknown(MidiMessage),
    CurrentPresetNumber(u32),
    CurrentPresetName(String),
}

pub fn parse_message(msg: MidiMessage) -> FractalMessage {
    let function_id = msg.iter().nth(5).unwrap();
    match function_id {
        20 => FractalMessage::CurrentPresetNumber(decode_preset_number(
            *msg.iter().nth(6).unwrap(),
            *msg.iter().nth(7).unwrap(),
        )),
        0x0F => {
            FractalMessage::CurrentPresetName(decode_preset_name(msg.into_iter().skip(6).collect()))
        }
        _ => FractalMessage::Unknown(msg),
    }
}

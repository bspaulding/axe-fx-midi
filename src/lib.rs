type MidiMessage = Vec<usize>;

enum FractalModel {
    Standard = 0x00,
    Ultra = 0x01,
    MFC101 = 0x02,
    II = 0x03,
    MFC101MK3 = 0x04,
    FX8 = 0x05,
    IIXL = 0x06,
    IIXLPlus = 0x07,
    AX8 = 0x08,
    FX8MK2 = 0x0A,
    III = 0x10,
}

// let header = vec![0xF0, 0x00, 0x01, 0x74];

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
}

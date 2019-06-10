mod parse;

pub use parse::{parse_message, FractalMessage,Effect,XYState,BlockFlags,BlockGridBlock, id_for_effect, Parameter};

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

pub fn disconnect_from_controller(model: FractalModel) -> MidiMessage {
    wrap_msg(vec![model_code(model), 0x42])
}

pub fn get_midi_channel(model: FractalModel) -> MidiMessage {
    wrap_msg(vec![model_code(model), 0x17])
}

pub enum TunerStatus { On, Off }

pub fn toggle_tuner(midi_channel: u32, tuner_status: TunerStatus) -> MidiMessage {
    vec![176 + (midi_channel - 1), 15, match tuner_status { TunerStatus::On => 127, TunerStatus::Off => 0 }]
}

pub enum MetronomeStatus { On, Off }

pub fn toggle_metronome(midi_channel: u32, status: MetronomeStatus) -> MidiMessage {
    vec![176 + (midi_channel - 1), 122, match status { MetronomeStatus::On => 127, MetronomeStatus::Off => 0 }]
}

pub fn get_preset_blocks_flags(model: FractalModel) -> MidiMessage {
    wrap_msg(vec![model_code(model), 0x0E])
}

pub fn set_scene_number(model: FractalModel, scene_number: u32) -> MidiMessage {
    wrap_msg(vec![model_code(model), 0x29, scene_number])
}

pub fn get_grid_layout_and_routing(model: FractalModel) -> MidiMessage {
    wrap_msg(vec![model_code(model), 0x20])
}

fn encode_effect_id(id: u32) -> (u32, u32) {
    (id & 0x7F, (id >> 7) & 0x7F)
}

pub fn get_block_parameters(model: FractalModel, effect: Effect) -> MidiMessage {
    let (a, b) = encode_effect_id(id_for_effect(effect));
    wrap_msg(vec![model_code(model), 0x01, a, b])
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

    #[test]
    fn test_disconnect_from_controller() {
        assert_eq!(
            vec![0xF0 ,0x00 ,0x01 ,0x74 ,model_code(FractalModel::II) ,0x42 ,68 ,0xF7],
            disconnect_from_controller(FractalModel::II)
        );
    }

    #[test]
    fn test_parse_front_panel_change_detected() {
        assert_eq!(
            parse_message(vec![240 ,0 ,1 ,116 ,3 ,0x21, 0, 0xF7]),
            FractalMessage::FrontPanelChangeDetected
        );
    }

    #[test]
    fn test_parse_midi_tempo_beat() {
        assert_eq!(
            parse_message(vec![240, 0 ,1 ,116 ,3 ,0x10 ,0xF7]),
            FractalMessage::MIDITempoBeat
        );
    }

    #[test]
    fn test_get_midi_channel() {
        assert_eq!(
            vec![240 ,0 ,1 ,116 ,3 ,0x17 ,17 ,0xF7],
            get_midi_channel(FractalModel::II)
        );
    }

    #[test]
    fn test_parse_midi_channel() {
        assert_eq!(
            parse_message(vec![240 ,0 ,1 ,116 ,3 ,0x17 ,9 ,0,0xF7]),
            FractalMessage::MIDIChannel(10)
        );
    }

    #[test]
    fn test_parse_tuner_info() {
        assert_eq!(
            parse_message(vec![240 ,0 ,1 ,116 ,3 ,0x0D ,1 ,2 ,63 ,0xF7]),
            FractalMessage::TunerInfo { note: 1, string_number: 2, tuner_data: 63 }
        );
    }

    #[test]
    fn test_toggle_tuner() {
        assert_eq!(
            vec![176, 15, 0],
            toggle_tuner(1, TunerStatus::Off)
        );
        assert_eq!(
            vec![177, 15, 0],
            toggle_tuner(2, TunerStatus::Off)
        );
        assert_eq!(
            vec![176, 15, 127],
            toggle_tuner(1, TunerStatus::On)
        );
    }

    #[test]
    fn test_toggle_metronome() {
        assert_eq!(
            vec![176, 122, 0],
            toggle_metronome(1, MetronomeStatus::Off)
        );
        assert_eq!(
            vec![177, 122, 0],
            toggle_metronome(2, MetronomeStatus::Off)
        );
        assert_eq!(
            vec![176, 122, 127],
            toggle_metronome(1, MetronomeStatus::On)
        );
    }

    #[test]
    fn test_get_preset_blocks_flags() {
        assert_eq!(
            vec![240, 0, 1, 116, 3, 0x0E, 8, 0xF7],
            get_preset_blocks_flags(FractalModel::II)
        );
    }

    #[test]
    fn test_parse_preset_blocks_flags() {
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 14, 3, 74, 16, 83, 6, 3, 78, 24, 99, 6, 2, 86, 124, 39, 6, 3, 94, 40, 3, 7, 2, 98, 48, 43, 120, 2, 100, 52, 51, 120, 3, 102, 124, 63, 120, 2, 10, 125, 23, 7, 3, 38, 81, 115, 6, 2, 52, 125, 7, 120, 3, 58, 125, 127, 7, 247]),
            FractalMessage::PresetBlocksFlags(vec![
                BlockFlags {
                    is_bypassed: false,
                    xy_state: XYState::X,
                    cc: 37,
                    effect_id: 106,
                    effect: Effect::Amp1
                },
                BlockFlags {
                    is_bypassed: false,
                    xy_state: XYState::X,
                    cc: 39,
                    effect_id: 108,
                    effect: Effect::Cab1
                },
                BlockFlags {
                    is_bypassed: true,
                    xy_state: XYState::X,
                    cc: 43,
                    effect_id: 100,
                    effect: Effect::Compressor1
                },
                BlockFlags {
                    is_bypassed: false,
                    xy_state: XYState::X,
                    cc: 47,
                    effect_id: 112,
                    effect: Effect::Delay1
                },
                BlockFlags {
                    is_bypassed: true,
                    xy_state: XYState::X,
                    cc: 49,
                    effect_id: 133,
                    effect: Effect::Drive1
                },
                BlockFlags {
                    is_bypassed: true,
                    xy_state: XYState::X,
                    cc: 50,
                    effect_id: 134,
                    effect: Effect::Drive2
                },
                BlockFlags {
                    is_bypassed: false,
                    xy_state: XYState::X,
                    cc: 51,
                    effect_id: 135,
                    effect: Effect::Enhancer
                },
                BlockFlags {
                    is_bypassed: true,
                    xy_state: XYState::X,
                    cc: 69,
                    effect_id: 114,
                    effect: Effect::MultiDelay1
                },
                BlockFlags {
                    is_bypassed: false,
                    xy_state: XYState::X,
                    cc: 83,
                    effect_id: 110,
                    effect: Effect::Reverb1
                },
                BlockFlags {
                    is_bypassed: true,
                    xy_state: XYState::X,
                    cc: 90,
                    effect_id: 128,
                    effect: Effect::TremoloPanner1
                },
                BlockFlags {
                    is_bypassed: false,
                    xy_state: XYState::X,
                    cc: 93,
                    effect_id: 127,
                    effect: Effect::VolumePan1
                },
            ])
        );
    }

    #[test]
    fn test_set_scene_number() {
        assert_eq!(
            vec![240, 0, 1, 116, 3, 41, 0, 47, 247],
            set_scene_number(FractalModel::II, 0)
        );
    }

    #[test]
    fn test_get_grid_layout_and_routing() {
        assert_eq!(
            vec![240, 0, 1, 116, 3, 0x20, 38, 0xF7],
            get_grid_layout_and_routing(FractalModel::II)
        );
    }

    #[test]
    fn test_parse_scene_number() {
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 41, 0, 47, 247]),
            FractalMessage::CurrentSceneNumber(1)
        );
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 41, 1, 47, 247]),
            FractalMessage::CurrentSceneNumber(2)
        );
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 41, 7, 47, 247]),
            FractalMessage::CurrentSceneNumber(8)
        );
    }

    #[test]
    fn test_parse_grid_layout_and_routing() {
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 32, 0, 0, 0, 0, 127, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 106, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 108, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 247]),
            FractalMessage::BlockGrid(
                [
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 127,
                            effect: Effect::VolumePan1,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 100,
                            effect: Effect::Compressor1,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 128,
                            effect: Effect::TremoloPanner1,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 133,
                            effect: Effect::Drive1,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 134,
                            effect: Effect::Drive2,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 106,
                            effect: Effect::Amp1,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 108,
                            effect: Effect::Cab1,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 207,
                            effect: Effect::Shunt,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 112,
                            effect: Effect::Delay1,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 114,
                            effect: Effect::MultiDelay1,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 110,
                            effect: Effect::Reverb1,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::EffectBlock {
                            effect_id: 135,
                            effect: Effect::Enhancer,
                            connect_row_1: false,
                            connect_row_2: true,
                            connect_row_3: false,
                            connect_row_4: false,
                        },
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                    [
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                        BlockGridBlock::Empty,
                    ],
                ]
            )
        );
    }

    #[test]
    fn test_get_block_parameters() {
        assert_eq!(
            vec![240, 0, 1, 116, 3, 0x01, 0, 1, 6, 0xF7],
            get_block_parameters(FractalModel::II, Effect::TremoloPanner1)
        );
        assert_eq!(
            vec![240, 0, 1, 116, 3, 0x01, 127, 0, 120, 0xF7],
            get_block_parameters(FractalModel::II, Effect::VolumePan1)
        );
        assert_eq!(
            vec![240, 0, 1, 116, 3, 0x01, 1, 1, 7, 0xF7],
            get_block_parameters(FractalModel::II, Effect::TremoloPanner2)
        );
    }

    #[test]
    fn test_parse_block_parameters() {
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 1, 106, 0, 0, 0, 109, 1, 0, 116, 0, 0, 2, 2, 65, 67, 45, 50, 48, 32, 49, 50, 65, 88, 55, 32, 66, 0, 55, 247]),
            FractalMessage::BlockParameters {
                effect_id: 106,
                effect: Effect::Amp1,
                parameter_id: 0,
                parameter: Parameter::EffectType,
                value_raw: 237
            }
        );
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 1, 106, 0, 1, 0, 75, 104, 0, 25, 0, 0, 0, 0, 50, 46, 48, 52, 0, 78, 247]),
            FractalMessage::BlockParameters {
                effect_id: 106,
                effect: Effect::Amp1,
                parameter_id: 1,
                parameter: Parameter::InputDrive,
                value_raw: 13387
            }
        );
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 1, 106, 0, 2, 0, 110, 70, 1, 49, 0, 0, 0, 0, 51, 46, 56, 56, 0, 106, 247]),
            FractalMessage::BlockParameters {
                effect_id: 106,
                effect: Effect::Amp1,
                parameter_id: 2,
                parameter: Parameter::Bass,
                value_raw: 25454
            }
        );
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 1, 106, 0, 3, 0, 25, 29, 3, 102, 0, 0, 0, 0, 56, 46, 48, 55, 0, 30, 247]),
            FractalMessage::BlockParameters {
                effect_id: 106,
                effect: Effect::Amp1,
                parameter_id: 3,
                parameter: Parameter::Middle,
                value_raw: 52889
            }
        );
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 1, 106, 0, 4, 0, 24, 51, 2, 76, 0, 0, 0, 0, 54, 46, 48, 48, 0, 20, 247]),
            FractalMessage::BlockParameters {
                effect_id: 106,
                effect: Effect::Amp1,
                parameter_id: 4,
                parameter: Parameter::Treble,
                value_raw: 39320
            }
        );
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 1, 106, 0, 5, 0, 126, 127, 3, 127, 0, 0, 0, 0, 49, 48, 46, 48, 48, 0, 58, 247]),
            FractalMessage::BlockParameters {
                effect_id: 106,
                effect: Effect::Amp1,
                parameter_id: 5,
                parameter: Parameter::MasterVolume,
                value_raw: 65534
            }
        );
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 1, 106, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 48, 46, 48, 32, 72, 122, 0, 102, 247]),
            FractalMessage::BlockParameters {
                effect_id: 106,
                effect: Effect::Amp1,
                parameter_id: 6,
                parameter: Parameter::PreampLowCut,
                value_raw: 0
            }
        );
        assert_eq!(
            parse_message(vec![240, 0, 1, 116, 3, 1, 106, 0, 7, 0, 118, 50, 3, 107, 0, 0, 0, 0, 50, 48, 48, 48, 48, 32, 72, 122, 0, 102, 247]),
            FractalMessage::BlockParameters {
                effect_id: 106,
                effect: Effect::Amp1,
                parameter_id: 7,
                parameter: Parameter::HighCutFrequency,
                value_raw: 55670
            }
        );
    }
}

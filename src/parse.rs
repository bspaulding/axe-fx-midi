use crate::MidiMessage;

#[derive(PartialEq, Debug, Clone, Copy)]
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

impl FractalModel {
    fn from_code(code: &u8) -> Option<Self> {
        match code {
            0x00 => Some(FractalModel::Standard),
            0x01 => Some(FractalModel::Ultra),
            0x02 => Some(FractalModel::MFC101),
            0x03 => Some(FractalModel::II),
            0x04 => Some(FractalModel::MFC101MK3),
            0x05 => Some(FractalModel::FX8),
            0x06 => Some(FractalModel::IIXL),
            0x07 => Some(FractalModel::IIXLPlus),
            0x08 => Some(FractalModel::AX8),
            0x0A => Some(FractalModel::FX8MK2),
            0x10 => Some(FractalModel::III),
            _ => None,
        }
    }
}

fn decode_preset_number(lsb: u8, rsb: u8) -> u32 {
    (((lsb as u32) & 0x7F) << 7) | (rsb as u32)
}

fn decode_preset_name(msg: Vec<u8>) -> String {
    msg.iter()
        .take(32)
        .filter(|x| *x > &0)
        .map(|x| *x as u8 as char)
        .collect::<String>()
        .trim_end()
        .to_string()
}

pub fn id_for_effect(effect: Effect) -> u8 {
    match effect {
        Effect::Compressor1 => 100,
        Effect::Compressor2 => 101,
        Effect::GraphicEQ1 => 102,
        Effect::GraphicEQ2 => 103,
        Effect::ParametricEQ1 => 104,
        Effect::ParametricEQ2 => 105,
        Effect::Amp1 => 106,
        Effect::Amp2 => 107,
        Effect::Cab1 => 108,
        Effect::Cab2 => 109,
        Effect::Reverb1 => 110,
        Effect::Reverb2 => 111,
        Effect::Delay1 => 112,
        Effect::Delay2 => 113,
        Effect::MultiDelay1 => 114,
        Effect::MultiDelay2 => 115,
        Effect::Chorus1 => 116,
        Effect::Chorus2 => 117,
        Effect::Flanger1 => 118,
        Effect::Flanger2 => 119,
        Effect::RotarySpeaker1 => 120,
        Effect::RotarySpeaker2 => 121,
        Effect::Phaser1 => 122,
        Effect::Phaser2 => 123,
        Effect::Wah1 => 124,
        Effect::Wah2 => 125,
        Effect::Formant => 126,
        Effect::VolumePan1 => 127,
        Effect::TremoloPanner1 => 128,
        Effect::TremoloPanner2 => 129,
        Effect::Pitch1 => 130,
        Effect::Filter1 => 131,
        Effect::Filter2 => 132,
        Effect::Drive1 => 133,
        Effect::Drive2 => 134,
        Effect::Enhancer => 135,
        Effect::FXLoop => 136,
        Effect::Mixer1 => 137,
        Effect::Mixer2 => 138,
        Effect::InputNoiseGate => 139,
        Effect::Output => 140,
        Effect::Controllers => 141,
        Effect::FeedbackSend => 142,
        Effect::FeedbackReturn => 143,
        Effect::Synth1 => 144,
        Effect::Synth2 => 145,
        Effect::Vocoder => 146,
        Effect::MegatapDelay => 147,
        Effect::Crossover1 => 148,
        Effect::Crossover2 => 149,
        Effect::GateExpander1 => 150,
        Effect::GateExpander2 => 151,
        Effect::Pitch2 => 153,
        Effect::MultibandCompressor1 => 154,
        Effect::MultibandCompressor2 => 155,
        Effect::QuadChorus1 => 156,
        Effect::QuadChorus2 => 157,
        Effect::Resonator1 => 158,
        Effect::Resonator2 => 159,
        Effect::GraphicEQ3 => 160,
        Effect::GraphicEQ4 => 161,
        Effect::ParametricEQ3 => 162,
        Effect::ParametricEQ4 => 163,
        Effect::Filter3 => 164,
        Effect::Filter4 => 165,
        Effect::VolumePan2 => 166,
        Effect::VolumePan3 => 167,
        Effect::VolumePan4 => 168,
        Effect::Looper => 169,
        Effect::Shunt => 207,
        _ => 0,
    }
}
fn effect_for_id(id: u32) -> Effect {
    match id {
        2 => Effect::Control,
        100 => Effect::Compressor1,
        101 => Effect::Compressor2,
        102 => Effect::GraphicEQ1,
        103 => Effect::GraphicEQ2,
        104 => Effect::ParametricEQ1,
        105 => Effect::ParametricEQ2,
        106 => Effect::Amp1,
        107 => Effect::Amp2,
        108 => Effect::Cab1,
        109 => Effect::Cab2,
        110 => Effect::Reverb1,
        111 => Effect::Reverb2,
        112 => Effect::Delay1,
        113 => Effect::Delay2,
        114 => Effect::MultiDelay1,
        115 => Effect::MultiDelay2,
        116 => Effect::Chorus1,
        117 => Effect::Chorus2,
        118 => Effect::Flanger1,
        119 => Effect::Flanger2,
        120 => Effect::RotarySpeaker1,
        121 => Effect::RotarySpeaker2,
        122 => Effect::Phaser1,
        123 => Effect::Phaser2,
        124 => Effect::Wah1,
        125 => Effect::Wah2,
        126 => Effect::Formant,
        127 => Effect::VolumePan1,
        128 => Effect::TremoloPanner1,
        129 => Effect::TremoloPanner2,
        130 => Effect::Pitch1,
        131 => Effect::Filter1,
        132 => Effect::Filter2,
        133 => Effect::Drive1,
        134 => Effect::Drive2,
        135 => Effect::Enhancer,
        136 => Effect::FXLoop,
        137 => Effect::Mixer1,
        138 => Effect::Mixer2,
        139 => Effect::InputNoiseGate,
        140 => Effect::Output,
        141 => Effect::Controllers,
        142 => Effect::FeedbackSend,
        143 => Effect::FeedbackReturn,
        144 => Effect::Synth1,
        145 => Effect::Synth2,
        146 => Effect::Vocoder,
        147 => Effect::MegatapDelay,
        148 => Effect::Crossover1,
        149 => Effect::Crossover2,
        150 => Effect::GateExpander1,
        151 => Effect::GateExpander2,
        153 => Effect::Pitch2,
        154 => Effect::MultibandCompressor1,
        155 => Effect::MultibandCompressor2,
        156 => Effect::QuadChorus1,
        157 => Effect::QuadChorus2,
        158 => Effect::Resonator1,
        159 => Effect::Resonator2,
        160 => Effect::GraphicEQ3,
        161 => Effect::GraphicEQ4,
        162 => Effect::ParametricEQ3,
        163 => Effect::ParametricEQ4,
        164 => Effect::Filter3,
        165 => Effect::Filter4,
        166 => Effect::VolumePan2,
        167 => Effect::VolumePan3,
        168 => Effect::VolumePan4,
        169 => Effect::Looper,
        207 => Effect::Shunt,
        _ => Effect::Unknown,
    }
}

fn chunk<T: Clone>(xs: Vec<T>, size: usize) -> Vec<Vec<T>> {
    let mut chunks = vec![];
    for i in (0..xs.len()).step_by(size) {
        if i + size < xs.len() {
            let mut chunk = vec![];
            for k in 0..size {
                chunk.push(xs[i + k].clone());
            }
            chunks.push(chunk);
        }
    }
    chunks
}

fn decode_effect_id(a: &u8, b: &u8) -> u32 {
    let a: u32 = (*a).into();
    let b: u32 = (*b).into();
    (a & 0x7F) | ((b & 0x7F) << 7)
}

fn decode_blocks_flags_effect_id(a: &u8, b: &u8) -> u32 {
    let a: u32 = (*a).into();
    let b: u32 = (*b).into();
    ((a & 0x78) >> 3) + ((b & 0x0F) << 4)
}

fn decode_preset_blocks_flags(msg: MidiMessage) -> Vec<BlockFlags> {
    chunk(msg, 5)
        .iter()
        .map(|chunk: &Vec<u8>| {
            let a = *chunk.iter().nth(0).unwrap();
            let b = *chunk.iter().nth(1).unwrap();
            let c = *chunk.iter().nth(2).unwrap();
            let d = *chunk.iter().nth(3).unwrap();
            let e = *chunk.iter().nth(4).unwrap();
            let effect_id = decode_blocks_flags_effect_id(&d, &e);
            BlockFlags {
                is_bypassed: !(a == 3 || a == 1),
                cc: (((b & 0x7E) >> 1) + ((c & 3) << 6)),
                effect_id: effect_id,
                effect: effect_for_id(effect_id),
                xy_state: if a == 3 || a == 2 {
                    XYState::X
                } else {
                    XYState::Y
                },
            }
        })
        .collect()
}

#[derive(PartialEq, Debug)]
pub enum XYState {
    X,
    Y,
}

#[derive(PartialEq, Debug)]
pub enum Effect {
    Amp1,
    Amp2,
    Cab1,
    Cab2,
    Chorus1,
    Chorus2,
    Compressor1,
    Compressor2,
    Control,
    Controllers,
    Crossover1,
    Crossover2,
    Delay1,
    Delay2,
    Drive1,
    Drive2,
    Enhancer,
    FeedbackReturn,
    FeedbackSend,
    Filter1,
    Filter2,
    Filter3,
    Filter4,
    Flanger1,
    Flanger2,
    Formant,
    FXLoop,
    GateExpander1,
    GateExpander2,
    GraphicEQ1,
    GraphicEQ2,
    GraphicEQ3,
    GraphicEQ4,
    InputNoiseGate,
    Looper,
    QuadChorus1,
    QuadChorus2,
    MegatapDelay,
    Mixer1,
    Mixer2,
    MultibandCompressor1,
    MultibandCompressor2,
    MultiDelay1,
    MultiDelay2,
    Output,
    Pitch1,
    Pitch2,
    Phaser1,
    Phaser2,
    ParametricEQ1,
    ParametricEQ2,
    ParametricEQ3,
    ParametricEQ4,
    Resonator1,
    Resonator2,
    Reverb1,
    Reverb2,
    RotarySpeaker1,
    RotarySpeaker2,
    Shunt,
    Synth1,
    Synth2,
    TremoloPanner1,
    TremoloPanner2,
    Vocoder,
    VolumePan1,
    VolumePan2,
    VolumePan3,
    VolumePan4,
    Wah1,
    Wah2,
    Unknown,
}

#[derive(PartialEq, Debug)]
pub enum Parameter {
    EffectType,
    InputDrive,
    Bass,
    Middle,
    Treble,
    MasterVolume,
    PreampLowCut,
    HighCutFrequency,
    Unknown,
}

#[derive(PartialEq, Debug)]
pub struct BlockFlags {
    pub is_bypassed: bool,
    pub xy_state: XYState,
    pub cc: u8,
    pub effect_id: u32,
    pub effect: Effect,
}

#[derive(PartialEq, Debug)]
pub enum BlockGridBlock {
    EffectBlock {
        effect_id: u32,
        effect: Effect,
        connect_row_1: bool,
        connect_row_2: bool,
        connect_row_3: bool,
        connect_row_4: bool,
    },
    Empty,
}

fn decode_block_grid_block(msg: &[u8]) -> BlockGridBlock {
    let a = &msg[0];
    let b = &msg[1];
    let c = &msg[2];
    // let d = &msg[3];
    let effect_id = decode_effect_id(a, b);
    match effect_id {
        0 => BlockGridBlock::Empty,
        _ => BlockGridBlock::EffectBlock {
            effect_id,
            effect: effect_for_id(effect_id),
            connect_row_1: 0 != (c & 1),
            connect_row_2: 0 != (c & 2),
            connect_row_3: 0 != (c & 4),
            connect_row_4: 0 != (c & 8),
        },
    }
}

fn decode_block_grid(msg: MidiMessage) -> [[BlockGridBlock; 4]; 16] {
    let cells = chunk(msg, 4);
    [
        [
            decode_block_grid_block(&cells[0][0..4]),
            decode_block_grid_block(&cells[1][0..4]),
            decode_block_grid_block(&cells[2][0..4]),
            decode_block_grid_block(&cells[3][0..4]),
        ],
        [
            decode_block_grid_block(&cells[4][0..4]),
            decode_block_grid_block(&cells[5][0..4]),
            decode_block_grid_block(&cells[6][0..4]),
            decode_block_grid_block(&cells[7][0..4]),
        ],
        [
            decode_block_grid_block(&cells[8][0..4]),
            decode_block_grid_block(&cells[9][0..4]),
            decode_block_grid_block(&cells[10][0..4]),
            decode_block_grid_block(&cells[11][0..4]),
        ],
        [
            decode_block_grid_block(&cells[12][0..4]),
            decode_block_grid_block(&cells[13][0..4]),
            decode_block_grid_block(&cells[14][0..4]),
            decode_block_grid_block(&cells[15][0..4]),
        ],
        [
            decode_block_grid_block(&cells[16][0..4]),
            decode_block_grid_block(&cells[17][0..4]),
            decode_block_grid_block(&cells[18][0..4]),
            decode_block_grid_block(&cells[19][0..4]),
        ],
        [
            decode_block_grid_block(&cells[20][0..4]),
            decode_block_grid_block(&cells[21][0..4]),
            decode_block_grid_block(&cells[22][0..4]),
            decode_block_grid_block(&cells[23][0..4]),
        ],
        [
            decode_block_grid_block(&cells[24][0..4]),
            decode_block_grid_block(&cells[25][0..4]),
            decode_block_grid_block(&cells[26][0..4]),
            decode_block_grid_block(&cells[27][0..4]),
        ],
        [
            decode_block_grid_block(&cells[28][0..4]),
            decode_block_grid_block(&cells[29][0..4]),
            decode_block_grid_block(&cells[30][0..4]),
            decode_block_grid_block(&cells[31][0..4]),
        ],
        [
            decode_block_grid_block(&cells[32][0..4]),
            decode_block_grid_block(&cells[33][0..4]),
            decode_block_grid_block(&cells[34][0..4]),
            decode_block_grid_block(&cells[35][0..4]),
        ],
        [
            decode_block_grid_block(&cells[36][0..4]),
            decode_block_grid_block(&cells[37][0..4]),
            decode_block_grid_block(&cells[38][0..4]),
            decode_block_grid_block(&cells[39][0..4]),
        ],
        [
            decode_block_grid_block(&cells[40][0..4]),
            decode_block_grid_block(&cells[41][0..4]),
            decode_block_grid_block(&cells[42][0..4]),
            decode_block_grid_block(&cells[43][0..4]),
        ],
        [
            decode_block_grid_block(&cells[44][0..4]),
            decode_block_grid_block(&cells[45][0..4]),
            decode_block_grid_block(&cells[46][0..4]),
            decode_block_grid_block(&cells[47][0..4]),
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
}

#[derive(Debug, PartialEq)]
pub enum TunerStatus {
    On,
    Off,
}

#[derive(PartialEq, Debug)]
pub enum FractalMessage {
    Unknown(MidiMessage),
    CurrentPresetNumber(u32),
    PresetName(u32, String),
    CurrentPresetName(String),
    CurrentSceneNumber(u8),
    CurrentTempo(u32),
    FirmwareVersion {
        major: u8,
        minor: u8,
    },
    FrontPanelChangeDetected,
    MIDITempoBeat,
    MIDIChannel(u8),
    TunerInfo {
        note: u8,
        string_number: u8,
        tuner_data: u8,
    },
    PresetBlocksFlags(Vec<BlockFlags>),
    BlockGrid([[BlockGridBlock; 4]; 16]),
    BlockParameters {
        effect_id: u32,
        effect: Effect,
        parameter_id: u32,
        parameter: Parameter,
        value_raw: u32,
    },
    TunerStatus(TunerStatus),
    MultipurposeResponse {
        function_id: u8,
        response_code: u8,
    },
}

fn parameter_for_id(id: u32) -> Parameter {
    match id {
        0 => Parameter::EffectType,
        1 => Parameter::InputDrive,
        2 => Parameter::Bass,
        3 => Parameter::Middle,
        4 => Parameter::Treble,
        5 => Parameter::MasterVolume,
        6 => Parameter::PreampLowCut,
        7 => Parameter::HighCutFrequency,
        _ => Parameter::Unknown,
    }
}

fn decode_parameter_value(a: u8, b: u8, c: u8) -> u32 {
    (a as u32 & 0x7F) | ((b as u32 & 0x7F) << 7) | ((c as u32 & 0x7F) << 14)
}

fn decode_block_parameters(msg: MidiMessage) -> FractalMessage {
    let effect_id = decode_effect_id(&msg.iter().nth(6).unwrap(), &msg.iter().nth(7).unwrap());
    let parameter_id = decode_effect_id(&msg.iter().nth(8).unwrap(), &msg.iter().nth(9).unwrap());
    FractalMessage::BlockParameters {
        effect_id,
        effect: effect_for_id(effect_id),
        parameter_id,
        parameter: parameter_for_id(parameter_id),
        value_raw: decode_parameter_value(
            *msg.iter().nth(10).unwrap(),
            *msg.iter().nth(11).unwrap(),
            *msg.iter().nth(12).unwrap(),
        ),
    }
}

// TODO: Parse multi-function response
pub fn parse_message(msg: MidiMessage) -> FractalMessage {
    let model: Option<FractalModel> = msg
        .iter()
        .nth(4)
        .map(FractalModel::from_code)
        .unwrap_or(None);
    let function_id = msg.iter().nth(5);
    match (model, function_id) {
        (Some(FractalModel::III), Some(0x14)) => FractalMessage::CurrentTempo(decode_effect_id(
            msg.iter().nth(6).unwrap(),
            msg.iter().nth(7).unwrap(),
        )),
        (_, Some(0x14)) => FractalMessage::CurrentPresetNumber(decode_preset_number(
            *msg.iter().nth(6).unwrap(),
            *msg.iter().nth(7).unwrap(),
        )),
        (_, Some(0x21)) => FractalMessage::FrontPanelChangeDetected,
        (_, Some(0x01)) => decode_block_parameters(msg),
        (_, Some(0x08)) => FractalMessage::FirmwareVersion {
            major: *msg.iter().nth(6).unwrap() as u8,
            minor: *msg.iter().nth(7).unwrap() as u8,
        },
        (Some(FractalModel::III), Some(0x0D)) => FractalMessage::PresetName(
            decode_effect_id(msg.iter().nth(6).unwrap(), msg.iter().nth(7).unwrap()),
            decode_preset_name(msg.into_iter().skip(8).collect()),
        ),
        (_, Some(0x0F)) => {
            FractalMessage::CurrentPresetName(decode_preset_name(msg.into_iter().skip(6).collect()))
        }
        (_, Some(0x10)) => FractalMessage::MIDITempoBeat,
        (_, Some(0x11)) => FractalMessage::TunerStatus(if *msg.iter().nth(6).unwrap() == 0 as u8 {
            TunerStatus::Off
        } else {
            TunerStatus::On
        }),
        (_, Some(0x17)) => FractalMessage::MIDIChannel(1 + *msg.iter().nth(6).unwrap() as u8),
        (_, Some(0x0D)) => FractalMessage::TunerInfo {
            note: *msg.iter().nth(6).unwrap() as u8,
            string_number: *msg.iter().nth(7).unwrap() as u8,
            tuner_data: *msg.iter().nth(8).unwrap() as u8,
        },
        (_, Some(0x0E)) => FractalMessage::PresetBlocksFlags(decode_preset_blocks_flags(
            msg.into_iter().skip(6).collect(),
        )),
        (_, Some(0x20)) => {
            FractalMessage::BlockGrid(decode_block_grid(msg.into_iter().skip(6).collect()))
        }
        (_, Some(0x29)) => {
            FractalMessage::CurrentSceneNumber(1 + *msg.iter().nth(6).unwrap() as u8)
        }
        (Some(FractalModel::III), Some(0x0C)) => {
            FractalMessage::CurrentSceneNumber(*msg.iter().nth(6).unwrap() as u8)
        }
        (_, Some(0x64)) => FractalMessage::MultipurposeResponse {
            function_id: *msg.iter().nth(6).unwrap() as u8,
            response_code: *msg.iter().nth(7).unwrap() as u8,
        },
        _ => FractalMessage::Unknown(msg),
    }
}

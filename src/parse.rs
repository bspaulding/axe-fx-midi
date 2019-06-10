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

fn effect_for_id(id: u32) -> Effect {
    match id {
        106 => Effect::Amp1,
        107 => Effect::Amp2,
        108 => Effect::Cab1,
        109 => Effect::Cab2,
        116 => Effect::Chorus1,
        117 => Effect::Chorus2,
        100 => Effect::Compressor1,
        101 => Effect::Compressor2,
        141 => Effect::Controllers,
        148 => Effect::Crossover1,
        149 => Effect::Crossover2,
        112 => Effect::Delay1,
        113 => Effect::Delay2,
        133 => Effect::Drive1,
        134 => Effect::Drive2,
        135 => Effect::Enhancer,
        136 => Effect::FXLoop,
        143 => Effect::FeedbackReturn,
        142 => Effect::FeedbackSend,
        131 => Effect::Filter1,
        132 => Effect::Filter2,
        164 => Effect::Filter3,
        165 => Effect::Filter4,
        118 => Effect::Flanger1,
        119 => Effect::Flanger2,
        126 => Effect::Formant,
        150 => Effect::GateExpander1,
        151 => Effect::GateExpander2,
        102 => Effect::GraphicEQ1,
        103 => Effect::GraphicEQ2,
        160 => Effect::GraphicEQ3,
        161 => Effect::GraphicEQ4,
        139 => Effect::InputNoiseGate,
        169 => Effect::Looper,
        147 => Effect::MegatapDelay,
        137 => Effect::Mixer1,
        138 => Effect::Mixer2,
        114 => Effect::MultiDelay1,
        115 => Effect::MultiDelay2,
        154 => Effect::MultibandCompressor1,
        155 => Effect::MultibandCompressor2,
        140 => Effect::Output,
        104 => Effect::ParametricEQ1,
        105 => Effect::ParametricEQ2,
        162 => Effect::ParametricEQ3,
        163 => Effect::ParametricEQ4,
        122 => Effect::Phaser1,
        123 => Effect::Phaser2,
        130 => Effect::Pitch1,
        153 => Effect::Pitch2,
        156 => Effect::QuadChorus1,
        157 => Effect::QuadChorus2,
        158 => Effect::Resonator1,
        159 => Effect::Resonator2,
        110 => Effect::Reverb1,
        111 => Effect::Reverb2,
        120 => Effect::RotarySpeaker1,
        121 => Effect::RotarySpeaker2,
        144 => Effect::Synth1,
        145 => Effect::Synth2,
        128 => Effect::TremoloPanner1,
        129 => Effect::TremoloPanner2,
        146 => Effect::Vocoder,
        127 => Effect::VolumePan1,
        166 => Effect::VolumePan2,
        167 => Effect::VolumePan3,
        168 => Effect::VolumePan4,
        124 => Effect::Wah1,
        125 => Effect::Wah2,
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

fn decode_effect_id(a: &u32, b: &u32) -> u32 {
    ((a & 0x78) >> 3) + ((b & 0x0F) << 4)
}

fn decode_preset_blocks_flags(msg: MidiMessage) -> Vec<BlockFlags> {
    chunk(msg, 5)
        .iter()
        .map(|chunk: &Vec<u32>| {
            let a = *chunk.iter().nth(0).unwrap();
            let b = *chunk.iter().nth(1).unwrap();
            let c = *chunk.iter().nth(2).unwrap();
            let d = *chunk.iter().nth(3).unwrap();
            let e = *chunk.iter().nth(4).unwrap();
            let effect_id = decode_effect_id(&d, &e);
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
pub struct BlockFlags {
    pub is_bypassed: bool,
    pub xy_state: XYState,
    pub cc: u32,
    pub effect_id: u32,
    pub effect: Effect,
}

#[derive(PartialEq, Debug)]
pub enum FractalMessage {
    Unknown(MidiMessage),
    CurrentPresetNumber(u32),
    CurrentPresetName(String),
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
}

// TODO: Parse multi-function response
pub fn parse_message(msg: MidiMessage) -> FractalMessage {
    let function_id = msg.iter().nth(5).unwrap();
    match function_id {
        20 => FractalMessage::CurrentPresetNumber(decode_preset_number(
            *msg.iter().nth(6).unwrap(),
            *msg.iter().nth(7).unwrap(),
        )),
        0x21 => FractalMessage::FrontPanelChangeDetected,
        0x08 => FractalMessage::FirmwareVersion {
            major: *msg.iter().nth(6).unwrap() as u8,
            minor: *msg.iter().nth(7).unwrap() as u8,
        },
        0x0F => {
            FractalMessage::CurrentPresetName(decode_preset_name(msg.into_iter().skip(6).collect()))
        }
        0x10 => FractalMessage::MIDITempoBeat,
        0x17 => FractalMessage::MIDIChannel(1 + *msg.iter().nth(6).unwrap() as u8),
        0x0D => FractalMessage::TunerInfo {
            note: *msg.iter().nth(6).unwrap() as u8,
            string_number: *msg.iter().nth(7).unwrap() as u8,
            tuner_data: *msg.iter().nth(8).unwrap() as u8,
        },
        0x0E => FractalMessage::PresetBlocksFlags(decode_preset_blocks_flags(
            msg.into_iter().skip(6).collect(),
        )),
        _ => FractalMessage::Unknown(msg),
    }
}

use crate::MidiMessage;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq)]
pub enum EffectID {
    ID_CONTROL = 2,
    ID_TUNER = 35,
    ID_IRCAPTURE,
    ID_INPUT1,
    ID_INPUT2,
    ID_INPUT3,
    ID_INPUT4,
    ID_INPUT5, // USB Input
    ID_OUTPUT1,
    ID_OUTPUT2,
    ID_OUTPUT3,
    ID_OUTPUT4,
    ID_COMP1,
    ID_COMP2,
    ID_COMP3,
    ID_COMP4,
    ID_GRAPHEQ1,
    ID_GRAPHEQ2,
    ID_GRAPHEQ3,
    ID_GRAPHEQ4,
    ID_PARAEQ1,
    ID_PARAEQ2,
    ID_PARAEQ3,
    ID_PARAEQ4,
    ID_DISTORT1,
    ID_DISTORT2,
    ID_DISTORT3,
    ID_DISTORT4,
    ID_CAB1,
    ID_CAB2,
    ID_CAB3,
    ID_CAB4,
    ID_REVERB1,
    ID_REVERB2,
    ID_REVERB3,
    ID_REVERB4,
    ID_DELAY1,
    ID_DELAY2,
    ID_DELAY3,
    ID_DELAY4,
    ID_MULTITAP1,
    ID_MULTITAP2,
    ID_MULTITAP3,
    ID_MULTITAP4,
    ID_CHORUS1,
    ID_CHORUS2,
    ID_CHORUS3,
    ID_CHORUS4,
    ID_FLANGER1,
    ID_FLANGER2,
    ID_FLANGER3,
    ID_FLANGER4,
    ID_ROTARY1,
    ID_ROTARY2,
    ID_ROTARY3,
    ID_ROTARY4,
    ID_PHASER1,
    ID_PHASER2,
    ID_PHASER3,
    ID_PHASER4,
    ID_WAH1,
    ID_WAH2,
    ID_WAH3,
    ID_WAH4,
    ID_FORMANT1,
    ID_FORMANT2,
    ID_FORMANT3,
    ID_FORMANT4,
    ID_VOLUME1,
    ID_VOLUME2,
    ID_VOLUME3,
    ID_VOLUME4,
    ID_TREMOLO1,
    ID_TREMOLO2,
    ID_TREMOLO3,
    ID_TREMOLO4,
    ID_PITCH1,
    ID_PITCH2,
    ID_PITCH3,
    ID_PITCH4,
    ID_FILTER1,
    ID_FILTER2,
    ID_FILTER3,
    ID_FILTER4,
    ID_FUZZ1,
    ID_FUZZ2,
    ID_FUZZ3,
    ID_FUZZ4,
    ID_ENHANCER1,
    ID_ENHANCER2,
    ID_ENHANCER3,
    ID_ENHANCER4,
    ID_MIXER1,
    ID_MIXER2,
    ID_MIXER3,
    ID_MIXER4,
    ID_SYNTH1,
    ID_SYNTH2,
    ID_SYNTH3,
    ID_SYNTH4,
    ID_VOCODER1,
    ID_VOCODER2,
    ID_VOCODER3,
    ID_VOCODER4,
    ID_MEGATAP1,
    ID_MEGATAP2,
    ID_MEGATAP3,
    ID_MEGATAP4,
    ID_CROSSOVER1,
    ID_CROSSOVER2,
    ID_CROSSOVER3,
    ID_CROSSOVER4,
    ID_GATE1,
    ID_GATE2,
    ID_GATE3,
    ID_GATE4,
    ID_RINGMOD1,
    ID_RINGMOD2,
    ID_RINGMOD3,
    ID_RINGMOD4,
    ID_MULTICOMP1,
    ID_MULTICOMP2,
    ID_MULTICOMP3,
    ID_MULTICOMP4,
    ID_TENTAP1,
    ID_TENTAP2,
    ID_TENTAP3,
    ID_TENTAP4,
    ID_RESONATOR1,
    ID_RESONATOR2,
    ID_RESONATOR3,
    ID_RESONATOR4,
    ID_LOOPER1,
    ID_LOOPER2,
    ID_LOOPER3,
    ID_LOOPER4,
    ID_TONEMATCH1,
    ID_TONEMATCH2,
    ID_TONEMATCH3,
    ID_TONEMATCH4,
    ID_RTA1,
    ID_RTA2,
    ID_RTA3,
    ID_RTA4,
    ID_PLEX1,
    ID_PLEX2,
    ID_PLEX3,
    ID_PLEX4,
    ID_FBSEND1,
    ID_FBSEND2,
    ID_FBSEND3,
    ID_FBSEND4,
    ID_FBRETURN1,
    ID_FBRETURN2,
    ID_FBRETURN3,
    ID_FBRETURN4,
    ID_MIDIBLOCK,
    ID_MULTIPLEXER1,
    ID_MULTIPLEXER2,
    ID_MULTIPLEXER3,
    ID_MULTIPLEXER4,
    ID_IRPLAYER1,
    ID_IRPLAYER2,
    ID_IRPLAYER3,
    ID_IRPLAYER4,
    ID_FOOTCONTROLLER,
    ID_PRESET_FC,
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
        Effect::Enhancer1 => 135,
        Effect::FXLoop => 136,
        Effect::Mixer1 => 137,
        Effect::Mixer2 => 138,
        Effect::InputNoiseGate => 139,
        Effect::Output => 140,
        Effect::Controllers => 141,
        Effect::FeedbackSend1 => 142,
        Effect::FeedbackReturn1 => 143,
        Effect::Synth1 => 144,
        Effect::Synth2 => 145,
        Effect::Vocoder1 => 146,
        Effect::MegatapDelay1 => 147,
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
        Effect::Looper1 => 169,
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
        135 => Effect::Enhancer1,
        136 => Effect::FXLoop,
        137 => Effect::Mixer1,
        138 => Effect::Mixer2,
        139 => Effect::InputNoiseGate,
        140 => Effect::Output,
        141 => Effect::Controllers,
        142 => Effect::FeedbackSend1,
        143 => Effect::FeedbackReturn1,
        144 => Effect::Synth1,
        145 => Effect::Synth2,
        146 => Effect::Vocoder1,
        147 => Effect::MegatapDelay1,
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
        169 => Effect::Looper1,
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Effect {
    Amp1,
    Amp2,
    Amp3,
    Amp4,
    Cab1,
    Cab2,
    Cab3,
    Cab4,
    Chorus1,
    Chorus2,
    Chorus3,
    Chorus4,
    Compressor1,
    Compressor2,
    Compressor3,
    Compressor4,
    Control,
    Controllers,
    Crossover1,
    Crossover2,
    Crossover3,
    Crossover4,
    Delay1,
    Delay2,
    Delay3,
    Delay4,
    Drive1,
    Drive2,
    Drive3,
    Drive4,
    Enhancer1,
    Enhancer2,
    Enhancer3,
    Enhancer4,
    FeedbackReturn1,
    FeedbackReturn2,
    FeedbackReturn3,
    FeedbackReturn4,
    FeedbackSend1,
    FeedbackSend2,
    FeedbackSend3,
    FeedbackSend4,
    Filter1,
    Filter2,
    Filter3,
    Filter4,
    Flanger1,
    Flanger2,
    Flanger3,
    Flanger4,
    FootController,
    Formant,
    Formant2,
    Formant3,
    Formant4,
    FXLoop,
    GateExpander1,
    GateExpander2,
    GateExpander3,
    GateExpander4,
    GraphicEQ1,
    GraphicEQ2,
    GraphicEQ3,
    GraphicEQ4,
    Input1,
    Input2,
    Input3,
    Input4,
    Input5,
    InputNoiseGate,
    IRCapture,
    IRPlayer1,
    IRPlayer2,
    IRPlayer3,
    IRPlayer4,
    Looper1,
    Looper2,
    Looper3,
    Looper4,
    MegatapDelay1,
    MegatapDelay2,
    MegatapDelay3,
    MegatapDelay4,
    MIDI,
    Mixer1,
    Mixer2,
    Mixer3,
    Mixer4,
    MultibandCompressor1,
    MultibandCompressor2,
    MultibandCompressor3,
    MultibandCompressor4,
    MultiDelay1,
    MultiDelay2,
    MultiDelay3,
    MultiDelay4,
    Multiplexer1,
    Multiplexer2,
    Multiplexer3,
    Multiplexer4,
    Output,
    Output1,
    Output2,
    Output3,
    Output4,
    ParametricEQ1,
    ParametricEQ2,
    ParametricEQ3,
    ParametricEQ4,
    Phaser1,
    Phaser2,
    Phaser3,
    Phaser4,
    Pitch1,
    Pitch2,
    Pitch3,
    Pitch4,
    PlexDelay1,
    PlexDelay2,
    PlexDelay3,
    PlexDelay4,
    PresetFC,
    QuadChorus1,
    QuadChorus2,
    RealtimeAnalyzer1,
    RealtimeAnalyzer2,
    RealtimeAnalyzer3,
    RealtimeAnalyzer4,
    Resonator1,
    Resonator2,
    Resonator3,
    Resonator4,
    Reverb1,
    Reverb2,
    Reverb3,
    Reverb4,
    RingModulator1,
    RingModulator2,
    RingModulator3,
    RingModulator4,
    RotarySpeaker1,
    RotarySpeaker2,
    RotarySpeaker3,
    RotarySpeaker4,
    Shunt,
    Synth1,
    Synth2,
    Synth3,
    Synth4,
    TenTapDelay1,
    TenTapDelay2,
    TenTapDelay3,
    TenTapDelay4,
    ToneMatch1,
    ToneMatch2,
    ToneMatch3,
    ToneMatch4,
    TremoloPanner1,
    TremoloPanner2,
    TremoloPanner3,
    TremoloPanner4,
    Tuner,
    Unknown,
    Vocoder1,
    Vocoder2,
    Vocoder3,
    Vocoder4,
    VolumePan1,
    VolumePan2,
    VolumePan3,
    VolumePan4,
    Wah1,
    Wah2,
    Wah3,
    Wah4,
}

impl Effect {
    fn from_effect_id(id: u32) -> Self {
        FromPrimitive::from_u32(id)
            .map(|effect_id| match effect_id {
                EffectID::ID_CONTROL => Self::Control,
                EffectID::ID_TUNER => Self::Tuner,
                EffectID::ID_IRCAPTURE => Self::IRCapture,
                EffectID::ID_INPUT1 => Self::Input1,
                EffectID::ID_INPUT2 => Self::Input2,
                EffectID::ID_INPUT3 => Self::Input3,
                EffectID::ID_INPUT4 => Self::Input4,
                EffectID::ID_INPUT5 => Self::Input5,
                EffectID::ID_OUTPUT1 => Self::Output1,
                EffectID::ID_OUTPUT2 => Self::Output2,
                EffectID::ID_OUTPUT3 => Self::Output3,
                EffectID::ID_OUTPUT4 => Self::Output4,
                EffectID::ID_COMP1 => Self::Compressor1,
                EffectID::ID_COMP2 => Self::Compressor2,
                EffectID::ID_COMP3 => Self::Compressor3,
                EffectID::ID_COMP4 => Self::Compressor4,
                EffectID::ID_GRAPHEQ1 => Self::GraphicEQ1,
                EffectID::ID_GRAPHEQ2 => Self::GraphicEQ2,
                EffectID::ID_GRAPHEQ3 => Self::GraphicEQ3,
                EffectID::ID_GRAPHEQ4 => Self::GraphicEQ4,
                EffectID::ID_PARAEQ1 => Self::ParametricEQ1,
                EffectID::ID_PARAEQ2 => Self::ParametricEQ2,
                EffectID::ID_PARAEQ3 => Self::ParametricEQ3,
                EffectID::ID_PARAEQ4 => Self::ParametricEQ4,
                EffectID::ID_DISTORT1 => Self::Amp1,
                EffectID::ID_DISTORT2 => Self::Amp2,
                EffectID::ID_DISTORT3 => Self::Amp3,
                EffectID::ID_DISTORT4 => Self::Amp4,
                EffectID::ID_CAB1 => Self::Cab1,
                EffectID::ID_CAB2 => Self::Cab2,
                EffectID::ID_CAB3 => Self::Cab3,
                EffectID::ID_CAB4 => Self::Cab4,
                EffectID::ID_REVERB1 => Self::Reverb1,
                EffectID::ID_REVERB2 => Self::Reverb2,
                EffectID::ID_REVERB3 => Self::Reverb3,
                EffectID::ID_REVERB4 => Self::Reverb4,
                EffectID::ID_DELAY1 => Self::Delay1,
                EffectID::ID_DELAY2 => Self::Delay2,
                EffectID::ID_DELAY3 => Self::Delay3,
                EffectID::ID_DELAY4 => Self::Delay4,
                EffectID::ID_MULTITAP1 => Self::MultiDelay1,
                EffectID::ID_MULTITAP2 => Self::MultiDelay2,
                EffectID::ID_MULTITAP3 => Self::MultiDelay3,
                EffectID::ID_MULTITAP4 => Self::MultiDelay4,
                EffectID::ID_CHORUS1 => Self::Chorus1,
                EffectID::ID_CHORUS2 => Self::Chorus2,
                EffectID::ID_CHORUS3 => Self::Chorus3,
                EffectID::ID_CHORUS4 => Self::Chorus4,
                EffectID::ID_FLANGER1 => Self::Flanger1,
                EffectID::ID_FLANGER2 => Self::Flanger2,
                EffectID::ID_FLANGER3 => Self::Flanger3,
                EffectID::ID_FLANGER4 => Self::Flanger4,
                EffectID::ID_ROTARY1 => Self::RotarySpeaker1,
                EffectID::ID_ROTARY2 => Self::RotarySpeaker2,
                EffectID::ID_ROTARY3 => Self::RotarySpeaker3,
                EffectID::ID_ROTARY4 => Self::RotarySpeaker4,
                EffectID::ID_PHASER1 => Self::Phaser1,
                EffectID::ID_PHASER2 => Self::Phaser2,
                EffectID::ID_PHASER3 => Self::Phaser3,
                EffectID::ID_PHASER4 => Self::Phaser4,
                EffectID::ID_WAH1 => Self::Wah1,
                EffectID::ID_WAH2 => Self::Wah2,
                EffectID::ID_WAH3 => Self::Wah3,
                EffectID::ID_WAH4 => Self::Wah4,
                EffectID::ID_FORMANT1 => Self::Formant,
                EffectID::ID_FORMANT2 => Self::Formant2,
                EffectID::ID_FORMANT3 => Self::Formant3,
                EffectID::ID_FORMANT4 => Self::Formant4,
                EffectID::ID_VOLUME1 => Self::VolumePan1,
                EffectID::ID_VOLUME2 => Self::VolumePan2,
                EffectID::ID_VOLUME3 => Self::VolumePan3,
                EffectID::ID_VOLUME4 => Self::VolumePan4,
                EffectID::ID_TREMOLO1 => Self::TremoloPanner1,
                EffectID::ID_TREMOLO2 => Self::TremoloPanner2,
                EffectID::ID_TREMOLO3 => Self::TremoloPanner3,
                EffectID::ID_TREMOLO4 => Self::TremoloPanner4,
                EffectID::ID_PITCH1 => Self::Pitch1,
                EffectID::ID_PITCH2 => Self::Pitch2,
                EffectID::ID_PITCH3 => Self::Pitch3,
                EffectID::ID_PITCH4 => Self::Pitch4,
                EffectID::ID_FILTER1 => Self::Filter1,
                EffectID::ID_FILTER2 => Self::Filter2,
                EffectID::ID_FILTER3 => Self::Filter3,
                EffectID::ID_FILTER4 => Self::Filter4,
                EffectID::ID_FUZZ1 => Self::Drive1, // TODO: wtf? assuming this maps to certain distortion types, within each drive block, so N refers to drive block N
                EffectID::ID_FUZZ2 => Self::Drive2,
                EffectID::ID_FUZZ3 => Self::Drive3,
                EffectID::ID_FUZZ4 => Self::Drive4,
                EffectID::ID_ENHANCER1 => Self::Enhancer1,
                EffectID::ID_ENHANCER2 => Self::Enhancer2,
                EffectID::ID_ENHANCER3 => Self::Enhancer3,
                EffectID::ID_ENHANCER4 => Self::Enhancer4,
                EffectID::ID_MIXER1 => Self::Mixer1,
                EffectID::ID_MIXER2 => Self::Mixer2,
                EffectID::ID_MIXER3 => Self::Mixer3,
                EffectID::ID_MIXER4 => Self::Mixer4,
                EffectID::ID_SYNTH1 => Self::Synth1,
                EffectID::ID_SYNTH2 => Self::Synth2,
                EffectID::ID_SYNTH3 => Self::Synth3,
                EffectID::ID_SYNTH4 => Self::Synth4,
                EffectID::ID_VOCODER1 => Self::Vocoder1,
                EffectID::ID_VOCODER2 => Self::Vocoder2,
                EffectID::ID_VOCODER3 => Self::Vocoder3,
                EffectID::ID_VOCODER4 => Self::Vocoder4,
                EffectID::ID_MEGATAP1 => Self::MegatapDelay1,
                EffectID::ID_MEGATAP2 => Self::MegatapDelay2,
                EffectID::ID_MEGATAP3 => Self::MegatapDelay3,
                EffectID::ID_MEGATAP4 => Self::MegatapDelay4,
                EffectID::ID_CROSSOVER1 => Self::Crossover1,
                EffectID::ID_CROSSOVER2 => Self::Crossover2,
                EffectID::ID_CROSSOVER3 => Self::Crossover3,
                EffectID::ID_CROSSOVER4 => Self::Crossover4,
                EffectID::ID_GATE1 => Self::GateExpander1,
                EffectID::ID_GATE2 => Self::GateExpander2,
                EffectID::ID_GATE3 => Self::GateExpander3,
                EffectID::ID_GATE4 => Self::GateExpander4,
                EffectID::ID_RINGMOD1 => Self::RingModulator1,
                EffectID::ID_RINGMOD2 => Self::RingModulator2,
                EffectID::ID_RINGMOD3 => Self::RingModulator3,
                EffectID::ID_RINGMOD4 => Self::RingModulator4,
                EffectID::ID_MULTICOMP1 => Self::MultibandCompressor1,
                EffectID::ID_MULTICOMP2 => Self::MultibandCompressor2,
                EffectID::ID_MULTICOMP3 => Self::MultibandCompressor3,
                EffectID::ID_MULTICOMP4 => Self::MultibandCompressor4,
                EffectID::ID_TENTAP1 => Self::TenTapDelay1,
                EffectID::ID_TENTAP2 => Self::TenTapDelay2,
                EffectID::ID_TENTAP3 => Self::TenTapDelay3,
                EffectID::ID_TENTAP4 => Self::TenTapDelay4,
                EffectID::ID_RESONATOR1 => Self::Resonator1,
                EffectID::ID_RESONATOR2 => Self::Resonator2,
                EffectID::ID_RESONATOR3 => Self::Resonator3,
                EffectID::ID_RESONATOR4 => Self::Resonator4,
                EffectID::ID_LOOPER1 => Self::Looper1,
                EffectID::ID_LOOPER2 => Self::Looper2,
                EffectID::ID_LOOPER3 => Self::Looper3,
                EffectID::ID_LOOPER4 => Self::Looper4,
                EffectID::ID_TONEMATCH1 => Self::ToneMatch1,
                EffectID::ID_TONEMATCH2 => Self::ToneMatch2,
                EffectID::ID_TONEMATCH3 => Self::ToneMatch3,
                EffectID::ID_TONEMATCH4 => Self::ToneMatch4,
                EffectID::ID_RTA1 => Self::RealtimeAnalyzer1,
                EffectID::ID_RTA2 => Self::RealtimeAnalyzer2,
                EffectID::ID_RTA3 => Self::RealtimeAnalyzer3,
                EffectID::ID_RTA4 => Self::RealtimeAnalyzer4,
                EffectID::ID_PLEX1 => Self::PlexDelay1,
                EffectID::ID_PLEX2 => Self::PlexDelay2,
                EffectID::ID_PLEX3 => Self::PlexDelay3,
                EffectID::ID_PLEX4 => Self::PlexDelay4,
                EffectID::ID_FBSEND1 => Self::FeedbackSend1,
                EffectID::ID_FBSEND2 => Self::FeedbackSend2,
                EffectID::ID_FBSEND3 => Self::FeedbackSend3,
                EffectID::ID_FBSEND4 => Self::FeedbackSend4,
                EffectID::ID_FBRETURN1 => Self::FeedbackReturn1,
                EffectID::ID_FBRETURN2 => Self::FeedbackReturn2,
                EffectID::ID_FBRETURN3 => Self::FeedbackReturn3,
                EffectID::ID_FBRETURN4 => Self::FeedbackReturn4,
                EffectID::ID_MIDIBLOCK => Self::MIDI,
                EffectID::ID_MULTIPLEXER1 => Self::Multiplexer1,
                EffectID::ID_MULTIPLEXER2 => Self::Multiplexer2,
                EffectID::ID_MULTIPLEXER3 => Self::Multiplexer3,
                EffectID::ID_MULTIPLEXER4 => Self::Multiplexer4,
                EffectID::ID_IRPLAYER1 => Self::IRPlayer1,
                EffectID::ID_IRPLAYER2 => Self::IRPlayer2,
                EffectID::ID_IRPLAYER3 => Self::IRPlayer3,
                EffectID::ID_IRPLAYER4 => Self::IRPlayer4,
                EffectID::ID_FOOTCONTROLLER => Self::FootController,
                EffectID::ID_PRESET_FC => Self::PresetFC,
            })
            .unwrap_or(Self::Unknown)
    }
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EffectStatus {
    pub effect_id: u32,
    pub effect_id_iii: Option<EffectID>,
    pub effect: Effect,
    pub bypassed: bool,
    pub channel: Channel,
    pub max_channels: u8,
}

#[derive(PartialEq, Debug)]
pub enum FractalMessage {
    Unknown(MidiMessage),
    StatusDump(Vec<EffectStatus>),
    LooperState {
        record: bool,
        play: bool,
        overdub: bool,
        once: bool,
        reverse: bool,
        half_speed: bool,
    },
    CurrentPresetNumber(u32),
    PresetName(u32, String),
    SceneName(u8, String),
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

fn parse_looper_state(byte: &u8) -> FractalMessage {
    FractalMessage::LooperState {
        record: byte & 0b00000001 != 0,
        play: byte & 0b00000010 != 0,
        overdub: byte & 0b00000100 != 0,
        once: byte & 0b00001000 != 0,
        reverse: byte & 0b00010000 != 0,
        half_speed: byte & 0b00100000 != 0,
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, FromPrimitive, PartialEq)]
pub enum Channel {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

fn parse_status_dump(bytes: Vec<u8>) -> FractalMessage {
    let effects: Vec<EffectStatus> = bytes
        .chunks_exact(3)
        .map(|packet| {
            let effect_id = decode_effect_id(&packet[0], &packet[1]);
            let effect = Effect::from_effect_id(effect_id);
            let dd = &packet[2];
            EffectStatus {
                effect_id,
                effect_id_iii: FromPrimitive::from_u32(effect_id),
                effect,
                bypassed: dd & 0b00000001 != 0,
                channel: FromPrimitive::from_u8(dd >> 1 & 0b00000111).unwrap(),
                max_channels: dd >> 4 & 0b0111,
            }
        })
        .collect();
    FractalMessage::StatusDump(effects)
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
        (Some(FractalModel::III), Some(0x13)) => {
            parse_status_dump(msg.into_iter().skip(6).collect())
        }
        (Some(FractalModel::III), Some(0x0F)) => parse_looper_state(msg.iter().nth(6).unwrap()),
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
        (Some(FractalModel::III), Some(0x0E)) => FractalMessage::SceneName(
            *msg.iter().nth(6).unwrap(),
            decode_preset_name(msg.into_iter().skip(7).collect()),
        ),
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

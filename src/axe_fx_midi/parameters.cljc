(ns axe-fx-midi.parameters)

(def group-by-effect-id
  {106 :amp
	 107 :amp
	 108 :cab
	 109 :cab
	 116 :chorus
	 117 :chorus
	 100 :compressor
	 101 :compressor
	 141 :controllers
	 148 :crossover
	 149 :crossover
	 112 :delay
	 113 :delay
	 131 :filter
	 132 :filter
	 133 :drive
	 134 :drive
	 135 :enhancer
	 136 :fx-loop
	 142 :feedback-send
	 143 :feedback-return
	 164 :filter
	 165 :filter
	 118 :flanger
	 119 :flanger
	 126 :formant
	 150 :gate-expander
	 151 :gate-expander
	 102 :graphic-eq
	 103 :graphic-eq
	 160 :graphic-eq
	 161 :graphic-eq
	 139 :input-noise-gate
	 169 :looper
	 147 :mega-tap-delay
	 137 :mixer
	 138 :mixer
	 114 :multi-delay
	 115 :multi-delay
	 154 :multi-band-compressor
	 155 :multi-band-compressor
	 140 :output
	 104 :parametric-eq
	 105 :parametric-eq
	 162 :parametric-eq
	 163 :parametric-eq
	 122 :phaser
	 123 :phaser
	 130 :pitch
	 153 :pitch
	 156 :quad-chorus
	 157 :quad-chorus
	 158 :resonator
	 159 :resonator
	 110 :reverb
	 111 :reverb
	 152 :ring-modulator
	 120 :rotary-speaker
	 121 :rotary-speaker
	 144 :synth
	 145 :synth
	 170 :tone-match
	 128 :tremolo-panner
	 129 :tremolo-panner
	 146 :vocoder
	 127 :volume
	 166 :volume
	 167 :volume
	 168 :volume
	 124 :wah
	 125 :wah})

(def parameters-by-group
  {:amp {0 "Effect Type"
         1 "Input Drive"
         2 "Bass"
         3 "Middle"
         4 "Treble"
         5 "Master Volume"
         6 "Preamp Low Cut"
         7 "High Cut Freq"
         8 "Tone Freq"
         9 "XFormer Grind"
         10 "Bright Cap"
         12 "XFormer Low Freq"
         13 "XFormer Hi Freq"
         14 "Tone Location"
         15 "Input Select"
         16 "Depth"
         19 "Supply Sag"
         20 "Presence"
         21 "Level"
         22 "Balance"
         23 "Bypass Mode"
         24 "Negative Feedback"
         25 "Presence Freq"
         26 "Low Res Freq"
         27 "Low Res"
         29 "Depth Freq"
         31 "MV Cap"
         33 "Harmonics"
         34 "Tone Stack"
         35 "B+ Time Const"
         36 "Tube Grid Bias"
         39 "Bright Switch"
         40 "Boost"
         41 "Low Res Q"
         42 "Preamp Bias"
         43 "Hi Freq"
         44 "Hi Resonance"
         45 "Cut"
         46 "XFormer Drive"
         47 "Input Trim"
         48 "Preamp Hardness"
         49 "MV Location"
         50 "Speaker Drive"
         51 "XFormer Match"
         54 "Saturation Switch"
         55 "GEQ Band 1"
         56 "GEQ Band 2"
         57 "GEQ Band 3"
         58 "GEQ Band 4"
         59 "GEQ Band 5"
         60 "GEQ Band 6"
         61 "GEQ Band 7"
         62 "GEQ Band 8"
         63 "Bias Excursion"
         66 "Triode 2 Plate Freq"
         67 "Triode 1 Plate Freq"
         68 "Power App Tube"
         69 "Preamp Tubes"
         70 "Out Comp Clarity"
         71 "Character Q"
         72 "Character Freq"
         73 "Character Amount"
         74 "Overdrive"
         75 "Out Comp Amount"
         76 "Out Comp Threshold"
         77 "Master Trim"
         78 "Fat"
         79 "Definition"
         80 "Preamp CF Compress"
         81 "Preamp CF Time"
         84 "Dynamic Presence"
         85 "Dynamic Depth"
         86 "Power Type"
         87 "AC Line Freq"
         88 "Power Amp Hardness"
         91 "Preamp CF Ratio"
         92 "EQ Type"
         93 "Cathode Resist"
         96 "Preamp Sag"
         97 "Bright"
         98 "Power Amp Bias"
         99 "Preamp Dynamics"
         100 "Hi Freq Slope"
         101 "Variac"
         102 "Char Type"
         104 "Presence Shift"
         105 "Saturation Drive"
         106 "Crunch"
         109 "Out Comp Type"
         110 "EQ Location"
         111 "CF Comp Type"
         113 "Preamp CF Hardness"
         114 "PI Bias Shift"
         115 "Motor Drive"
         116 "Motor Time Const"}
    :volume {0 "Volume"
						 1 "Balance"
						 2 "Volume Taper"
						 4 "Pan Left"
						 5 "Pan Right"
						 6 "Level"
						 7 "Bypass Mode"
						 8 "Input Select"}})

(defn parameter-name [effect-id parameter-id]
  (get-in parameters-by-group [(group-by-effect-id effect-id) parameter-id]))

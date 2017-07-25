(ns axe-fx-midi.blocks)

(def blocks
  {106 "Amp 1"
   107 "Amp 2"
   108 "Cab 1"
   109 "Cab 2"
   116 "Chorus 1"
   117 "Chorus 2"
   100 "Compressor 1"
   101 "Compressor 2"
   141 "Controllers"
   148 "Crossover 1"
   149 "Crossover 2"
   112 "Delay 1"
   113 "Delay 2"
   133 "Drive 1"
   134 "Drive 2"
   135 "Enhancer"
   136 "FX Loop"
   143 "Feedback Return"
   142 "Feedback Send"
   131 "Filter 1"
   132 "Filter 2"
   164 "Filter 3"
   165 "Filter 4"
   118 "Flanger 1"
   119 "Flanger 2"
   126 "Format"
   150 "Gate Expander"
   151 "Gate Expander 2"
   102 "Graphic EQ 1"
   103 "Graphic EQ 2"
   160 "Graphic EQ 3"
   161 "Graphic EQ 4"
   139 "Input Noise Gate"
   169 "Looper"
   147 "Megatap Delay"
   137 "Mixer"
   138 "Mixer 2"
   114 "Multi Delay 1"
   115 "Multi Delay 2"
   154 "Multiband Compressor 1"
   155 "Multiband Compressor 2"
   140 "Output"
   104 "Parametric EQ 1"
   105 "Parametric EQ 2"
   162 "Parametric EQ 3"
   163 "Parametric EQ 4"
   122 "Phaser 1"
   123 "Phaser 2"
   130 "Pitch 1"
   153 "Pitch 2"
   156 "Quad Chorus 1"
   157 "Quad Chorus 2"
   158 "Resonator 1"
   159 "Resonator 2"
   110 "Reverb 1"
   111 "Reverb 2"
   120 "Rotary Speaker 1"
   121 "Rotary Speaker 2"
   144 "Synth 1"
   145 "Synth 2"
   128 "Tremolo/Panner 1"
   129 "Tremolo/Panner 2"
   146 "Vocoder"
   127 "Volume/Pan 1"
   166 "Volume/Pan 2"
   167 "Volume/Pan 3"
   168 "Volume/Pan 4"
   124 "Wah 1"
   125 "Wah 2"})

(defn name-for-effect-id [effect-id]
  (cond
    (and (>= effect-id 200) (<= effect-id 235)) "Shunt"
    (and (>= effect-id 300) (<= effect-id 335)) "Shunt"
    :else (get blocks effect-id)))
(ns axe-fx-midi.ascii)

(defn string-to-ascii [xs]
  #?(:cljs (map #(.charCodeAt % 0) xs)
     :clj (map int xs)))

(defn safe-string-to-ascii
  "Removes unsupported characters"
  [xs]
  (filter #(< % 127) (string-to-ascii xs)))

(defn ascii-to-string [xs]
  (apply str (map #?(:cljs #(.fromCharCode js/String %)
                     :clj char)
                  xs)))

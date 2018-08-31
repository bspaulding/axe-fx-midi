(ns axe-fx-midi.ascii-test
  (:require #?(:cljs [cljs.test :as t :refer-macros [deftest testing is]]
               :clj  [clojure.test :as t :refer [deftest testing is]])
            [axe-fx-midi.ascii :refer [string-to-ascii safe-string-to-ascii ascii-to-string]]))

(deftest test-string-to-ascii []
  (is (= '(97 115 99 105 105) (string-to-ascii "ascii"))))

(deftest test-safe-string-to-ascii []
  (is (= "O Praise The Name (Anstasis)"
         (-> "O Praise The Name (Anástasis)"
             safe-string-to-ascii
             ascii-to-string))))

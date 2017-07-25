(set-env!
 :source-paths #{"src"}
 :dependencies '[[adzerk/boot-cljs          "2.0.0"      :scope "test"]
                 [adzerk/boot-cljs-repl     "0.3.3"      :scope "test"]
                 [pandeiro/boot-http        "0.8.3"      :scope "test"]
                 [com.cemerick/piggieback   "0.2.1"      :scope "test"]
                 [org.clojure/tools.nrepl   "0.2.13"     :scope "test"]
                 [org.clojure/clojurescript "1.9.562"]
                 [crisptrutski/boot-cljs-test "0.3.0" :scope "test"]])

(require
 '[adzerk.boot-cljs      :refer [cljs]]
 '[adzerk.boot-cljs-repl :refer [cljs-repl start-repl]]
 '[crisptrutski.boot-cljs-test :refer [test-cljs]])

(task-options!
  pom {:project 'org.clojars.bspaulding/axe-fx-midi
       :version "1.0.0-SNAPSHOT"
       :description "Generate and parse MIDI messages for interacting with Fractal Axe-Fx"
       :url "https://github.com/bspaulding/axe-fx-midi"
       :scm {:url "https://github.com/bspaulding/axe-fx-midi"}})

(deftask build []
  (cljs {:source-map true}))

(deftask testing []
  (set-env! :source-paths #(conj % "test"))
  identity)

(deftask tdd []
  (comp (watch)
        (test)))

;;; This prevents a name collision WARNING between the test task and
;;; clojure.core/test, a function that nobody really uses or cares
;;; about.
(ns-unmap 'boot.user 'test)

(deftask test []
  (comp (testing)
        (test-cljs :js-env :node
                   :exit?  true)))
(deftask install-jar []
  (merge-env! :resource-paths #{"src"})
  (comp
    (pom)
    (jar)
    (install)))

(set-env!
 :source-paths #{"src"}
 :resource-paths #{"src"}
 :dependencies '[[adzerk/boot-cljs            "2.0.0"      :scope "test"]
                 [adzerk/boot-cljs-repl       "0.3.3"      :scope "test"]
                 [pandeiro/boot-http          "0.8.3"      :scope "test"]
                 [com.cemerick/piggieback     "0.2.1"      :scope "test"]
                 [org.clojure/tools.nrepl     "0.2.13"     :scope "test"]
                 [org.clojure/clojurescript   "1.9.562"]
                 [crisptrutski/boot-cljs-test "0.3.0"      :scope "test"]
								 [adzerk/boot-test            "1.2.0"      :scope "test"]
								 [adzerk/bootlaces						"0.1.13"		 :scope "test"]])

(require
 '[adzerk.boot-cljs      :refer [cljs]]
 '[adzerk.boot-cljs-repl :refer [cljs-repl start-repl]]
 '[crisptrutski.boot-cljs-test :refer [test-cljs]]
 '[adzerk.boot-test :refer :all]
 '[adzerk.bootlaces :refer :all])

(def version "1.0.0-SNAPSHOT")

(bootlaces! version)

(task-options!
  pom {:project 'bspaulding/axe-fx-midi
       :version version
       :description "Generate and parse MIDI messages for interacting with Fractal Axe-Fx"
       :url "https://github.com/bspaulding/axe-fx-midi"
       :scm {:url "https://github.com/bspaulding/axe-fx-midi"}}
	test-cljs {:js-env :node})

(deftask build []
  (cljs {:source-map true}))

(deftask testing []
  (set-env! :source-paths #(conj % "test"))
  identity)

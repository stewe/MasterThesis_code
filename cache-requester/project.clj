(defproject cache-requester "0.1.0-SNAPSHOT"
  :description "A simple requester for testing the cache api."
  :url "http://example.com/FIXME"
  :license {:name "Eclipse Public License"
            :url "http://www.eclipse.org/legal/epl-v10.html"}
  :dependencies [[org.clojure/clojure "1.7.0"],
                  [org.zeromq/cljzmq "0.1.4"]]
  :main ^:skip-aot cache-requester.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}}
  :jvm-opts ["-Djava.library.path=/usr/local/lib"])

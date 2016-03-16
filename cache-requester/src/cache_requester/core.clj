(ns cache-requester.core
  (:gen-class)
  (:require [zeromq.zmq :as zmq]))

  (def caller-id "clj-requester")
  (def addr "tcp://127.0.0.1:5550")
  (def req-get "get INF (sensor-java, a, 3)")
  (def req-add "ADD sensor-clj 1 2")
  (def req-add-filter "ADD sensor-clj 0")

  ; TODO think about cache API get:
  ; GET duration filter1 amount1 filter2 amount2 ...
  ; now: one queue per sensor ->

  (defn -main [& args]
    ; (println (= (first args) "add"))
    (let [context (zmq/context 1),
          req (case (first args)
                    "add" req-add
                    "addf" req-add-filter
                    "get" req-get
                    nil req-get)]
      (println "Connecting to the cache…")
      (with-open [socket (doto (zmq/socket context :req)
                                  (zmq/connect addr))]
                          (println "Sending " req "…")
                          (zmq/send-str socket req)
                          (println "Received " (String. (zmq/receive socket))))))

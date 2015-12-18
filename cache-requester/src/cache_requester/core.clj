(ns cache-requester.core
  (:gen-class)
  (:require [zeromq.zmq :as zmq]))

  (def caller-id "clj-requester")
  (def addr "tcp://127.0.0.1:5550")
  (def req-get "get INF (sensor-java, a, 3)")

  ; TODO think about cache API get:
  ; GET duration filter1 amount1 filter2 amount2 ...
  ; now: one queue per sensor ->

  (defn -main []
    (let [context (zmq/context 1)]
      (println "Connecting to the cache…")
      (with-open [socket (doto (zmq/socket context :req)
                                  (zmq/connect addr))]
                          (println "Sending " req-get "…")
                          (zmq/send-str socket req-get)
                          (println "Received " (String. (zmq/receive socket))))))

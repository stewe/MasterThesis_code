(ns cache-requester.core
  (:gen-class)
  (:require [zeromq.zmq :as zmq]))

  ; (def caller-id "clj-requester")
  (def addr "tcp://127.0.0.1:5550")
  ; (def req-get "get INF (sensor-java, a, 3)")
  ; (def req-add "ADD sensor-clj 1 2")
  ; (def req-add-filter "ADD sensor-clj 0")
  (def msg (String. (byte-array [123, 34, 109, 115, 103, 95, 116, 121, 112, 101, 34, 58, 34, 83, 85, 66, 34, 44, 34, 109, 115, 103, 34, 58, 91, 49, 50, 51, 44, 51, 52, 44, 49, 49, 56, 44, 57, 55, 44, 49, 48, 56, 44, 51, 52, 44, 53, 56, 44, 57, 49, 44, 57, 49, 44, 53, 55, 44, 53, 55, 44, 52, 52, 44, 52, 57, 44, 52, 56, 44, 53, 54, 44, 52, 52, 44, 53, 55, 44, 53, 53, 44, 52, 52, 44, 52, 57, 44, 52, 56, 44, 53, 55, 44, 52, 52, 44, 52, 57, 44, 52, 57, 44, 53, 48, 44, 52, 52, 44, 53, 50, 44, 53, 55, 44, 52, 52, 44, 53, 51, 44, 53, 49, 44, 57, 51, 44, 52, 52, 44, 57, 49, 44, 52, 57, 44, 52, 57, 44, 53, 53, 44, 52, 52, 44, 52, 57, 44, 52, 57, 44, 52, 56, 44, 52, 52, 44, 53, 55, 44, 53, 55, 44, 52, 52, 44, 52, 57, 44, 52, 56, 44, 53, 54, 44, 52, 52, 44, 52, 57, 44, 52, 57, 44, 53, 53, 44, 52, 52, 44, 52, 57, 44, 52, 57, 44, 53, 52, 44, 52, 52, 44, 53, 55, 44, 53, 55, 44, 52, 52, 44, 52, 57, 44, 52, 56, 44, 53, 50, 44, 57, 51, 44, 57, 51, 44, 49, 50, 53, 93, 44, 34, 99, 108, 105, 101, 110, 116, 95, 105, 100, 34, 58, 110, 117, 108, 108, 44, 34, 109, 97, 99, 34, 58, 110, 117, 108, 108, 44, 34, 116, 105, 109, 101, 34, 58, 49, 52, 54, 55, 48, 52, 54, 50, 57, 53, 55, 50, 51, 125])))

  ; TODO think about cache API get:
  ; GET duration filter1 amount1 filter2 amount2 ...
  ; now: one queue per sensor ->

  (defn -main [& args]
    ; (println (= (first args) "add"))
    (let [context (zmq/context 1),
          ; req (reduce (fn [v x] (str v " " x)) args)
            ; (case (first args)
            ;         "add" req-add
            ;         "addf" req-add-filter
            ;         "get" req-get
            ;         nil req-get)
                    ]
      (println "Connecting to the cache…")
      (with-open [socket (doto (zmq/socket context :req)
                                  (zmq/connect addr))]
                          ; (println "Sending " req "…")
                          ; (zmq/send-str socket req)
                          (println "Sending msg" msg)
                          (zmq/send-str socket msg)
                          (println "Received " (String. (zmq/receive socket))))))

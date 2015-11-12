(ns sensor-clj.core
  (:gen-class)
  (:require [zeromq.zmq :as zmq]))

(def addr "tcp://*:5556")

(defn -main []
  (println "Sensor started on " addr)
  (let [context (zmq/zcontext)]
    (with-open [publisher (doto (zmq/socket context :pub)
                                (zmq/bind addr))]
    (while (not (.. Thread currentThread isInterrupted))
          (let [zipcode (rand-int 3)
                temperature (- (rand-int 215) 80)
                relhumidity (+ (rand-int 50) 10)
                msg (format "%d %d %d" zipcode temperature relhumidity)]
            (zmq/send-str publisher msg)
            (println "sent" msg)
            (Thread/sleep 10000)
            ))
            )))

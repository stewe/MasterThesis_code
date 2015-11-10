(ns sensor-clj.core
  (:gen-class)
  (:require [zeromq.zmq :as zmq]))

(defn -main []
  (println "Sensor started.")
  (let [context (zmq/zcontext)]
    (with-open [publisher (doto (zmq/socket context :pub)
                                (zmq/bind "tcp://*:5556"))]
    (while (not (.. Thread currentThread isInterrupted))
          (let [zipcode (rand-int 100000)
                temperature (- (rand-int 215) 80)
                relhumidity (+ (rand-int 50) 10)
                msg (format "%05d %d %d" zipcode temperature relhumidity)]
            (zmq/send-str publisher msg)
            (println "sent" msg)
            (Thread/sleep 10000)
            ))
            )))

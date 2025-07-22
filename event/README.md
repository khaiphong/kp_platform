# event

We use socket <a href="https://github.com/cloudevents/spec" target="_blank">CloudEvents</a>, a specification for describing event data, within a <a href="https://github.com/cloudevents/spec/blob/main/cloudevents/bindings/websockets-protocol-binding.md" target="_blank">WebSocket connection</a>. This means events are exchanged between systems / containers using a full-duplex WebSocket stream, where the event metadata and data are serialized according to the CloudEvents specification and transmitted as WebSocket messages. 

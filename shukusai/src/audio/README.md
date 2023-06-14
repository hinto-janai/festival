# Audio
`Audio` loops over the samples of an audio file, demuxing and decoding each packet, and writes the data to the audio hardware.

| File           | Purpose |
|----------------|---------|
| audio.rs       | Main `Audio` loop
| msg.rs         | Types of messages `Audio` and `Kernel` can send to each other

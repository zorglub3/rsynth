# Synth Engine in Rust

A simple set of modules and a simulator for a virtual analogue modular synth. Currently, 
the modules can only be set up at compile time, but in the future, this may change. The
engine supports this.

It features a RK4 simulator. This can be replaced for a different RK tableau if better 
accuracy or performance is desired. The engine is pretty light weight, so it should be 
possible to port it to some simple microcontroller board for a small simple hardware 
synth.

## Future work

- Improve sound of the oscillator module. Right now it suffers from aliasing problems
  and the reflection of harmonics at the Nyquist frequency.
- More modules... Especially for control, but also delay and some reverbs
- Improved support for various scales. Just intonation. That sort of thing.

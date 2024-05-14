# Synth Engine in Rust

A simple set of modules and a simulator for a virtual analogue modular synth. Currently, 
the modules can only be set up at compile time, but in the future, this may change. The
engine supports this.

## Future work

- Improve sound of the oscillator module. Right now it suffers from aliasing problems
  and the reflection of harmonics at the Nyquist frequency.
- More modules... Especially for control, but also delay and some reverbs
- Improved support for various scales. Just intonation. That sort of thing.

## Testing

The project includes a simple test tool for running on the command line. The intend
is to test the quality of the output produced by the synth engine by visual inspection.

For example, run this:

```bash
cargo run --bin cli-test \
  -- -t 0 -c 1024 -s 44100 --simulator rk38 \
  |  gnuplot -e "plot '-' w lp; pause 99"
```

And see the output of the saw tooth oscillator when frequency approaches the Nyquist 
frequency. It should show something more like a sine wave than a saw tooth.

## References



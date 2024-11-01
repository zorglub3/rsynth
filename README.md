# Synth Engine in Rust

A simple set of modules and a simulator for a virtual analogue modular synth. 
The synthesizer uses "Unsampled Digital Synthesis". It uses Runge Kutta methods
for running the simulation. This method is good for simulating systems that can
be described with partial differential equations. It is very bad at simulating
systems with break points.

Currently, the modules can only be set up at compile time, but in the future,
this may change. Adding and removing modules while the simulation is running
should be possilbe. Also, connecting and disconnecting modules should be
possible as well. 

## Future work

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

## Running the synth

Use the `cli-synth` command to run a synth. Run the program to see a list of command line
options. Most important is the `--model` (or `-m`) option to select synth model. Use INI
files in the `synths` directory for inspiration.

Use `aconnect -i` and `aconnect -o` to see a list of input and output MIDI devices on
your system. Then use `aconnect <Midi controller> rsynth` to connect your controller to 
the running instance of the synth. Play music and enjoy!

## References

- Unsampled Digital Synthesis: Computing the Output of Implicit and Non-Linear
  Systems, David Medine, 2015

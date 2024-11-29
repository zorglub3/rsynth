# Synth Engine in Rust

A simple set of modules and a simulator for a virtual analogue modular synth. 
The synthesizer uses "Unsampled Digital Synthesis". It uses Runge Kutta methods
for running the simulation. This method is good for simulating systems that can
be described with partial differential equations. It is very bad at simulating
systems with break points.

The configuration of the synthesizer system, the modules and interconnections,
can be defined with an INI file. The command line synth reads such a file and 
simulates the synthesizer system. It can receive MIDI events and play sound.

## Future work

- More modules... Especially for control, but also delay and some reverbs.
- Improved "input expressions" for defining module interconnections.
- Compile time parsing of INI files to generate Rust code for defining a synth.
- Compile to more exotic targets: Raspberry Pi and Daisy Seed. 
- Build unikernel deployments of a complete synth to Raspberry Pi to make a 
  "hardware synth" that starts up real quick. Have a look at unikraft.
- More testing.
- Documentation.

### Stuff to do soon

- Improve/simplify the wavetable module
- Use PolyBLEP for the sawtooth oscillator (and make a PWM/square oscillator).
- Clean up modules. Some are redundant (quadrature oscillator), some should be
  renamed (`midi_cc` and `midi_mono` - they are not really MIDI), some of the 
  code can probably be improved a lot.

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
- Source of single cycle wav-files for the wavetable oscillator, see the
  [AKWF-FREE repository](https://github.com/KristofferKarlAxelEkstrand/AKWF-FREE.git).
- For a quick introduction to waveguide synthesis, look [here](https://www.osar.fr/notes/waveguides/)
- A thorough exposition on physical modelling, is [here](https://ccrma.stanford.edu/~jos/) on 
  Julius Orion Smith III homepage.

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

- Compile time parsing of INI files to generate Rust code for defining a synth.
- Compile to more exotic targets: Raspberry Pi and Daisy Seed. Use `no_std` in
  `synth-engine` and allow for INI file to Rust compilation at build time.
- Build unikernel deployments of a complete synth to Raspberry Pi to make a 
  "hardware synth" that starts up real quick. Have a look at unikraft.
- More testing.
- Documentation.
- Allow for toplevel expression definitions in INI files. To work like "define"
  in eg C/C++.

### More modules

- Simple Sum and Integral modules for composing more advanced modules without 
  recompiling everything.
- Allpass filters and cascaded allpass filters. To build waveguide modules.
- Highpass filter (6db).
- DC reject filter.
- Various reed models, bowed string, et.c.
- Vosim variant on wavetable module

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

## Example synth with code generation

Commands I used after adding the workspace to the `Cargo.toml` file:

```bash
cargo new --bin example
cargo add synth-engine --path synth-engine --package example
cargo add --build synth-designer --path synth-designer --package example
touch example/build.rs
```

Use `rustfmt` to clean up the output.

## References

- Unsampled Digital Synthesis: Computing the Output of Implicit and Non-Linear
  Systems, David Medine, 2015
- Source of single cycle wav-files for the wavetable oscillator, see the
  [AKWF-FREE repository](https://github.com/KristofferKarlAxelEkstrand/AKWF-FREE.git).
- For a quick introduction to waveguide synthesis, look [here](https://www.osar.fr/notes/waveguides/)
- A thorough exposition on physical modelling, is [here](https://ccrma.stanford.edu/~jos/) on 
  Julius Orion Smith III homepage.
- Dynamical Systems for Audio Synthesis: Embracing Nonlinearities and Delay-Free Loops,
  David Medine, 2015
- A REVIEW OF METHODS FOR RESOLVING DELAY-FREE LOOPS, Jatin Chowdhury, (don't know when it
  was publicized)
- A COMPARISON OF VIRTUAL ANALOG MODELLING TECHNIQUES FOR DESKTOP AND EMBEDDED IMPLEMENTATIONS,
  Jatin Chowdhury, 2020

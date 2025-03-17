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
  _This is work in progress. The basics are there, but this needs testing._
- Compile to more exotic targets: Raspberry Pi and Daisy Seed. Use `no_std` in
  `synth-engine`. _This is work in progress. For now I'm trying out the Daisy Seed._
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

## Notes on daisy seed

When it all goes wrong and even the boot loader is broken, then:

1. Connect the daisy seed to the pc.
2. Press and hold `boot` button.
3. Press and release `reset button.
4. Release `boot` button. One daisy seed LED should light up lots then just be lit.
5. Go to the `core` folder in `libDaisy` (as found on github).
6. Run `make program-boot`

To upload something do as described [here](https://forum.electro-smith.com/t/resolved-error-when-using-program-dfu-with-diasy-bootloader-and-the-app-type-sram/3934/3):

1. Press and release `reset` button.
2. Press and release `boot` button. LED should "breathe" slowly. Boot loader is ready.
3. Upload `.bin` file with the `bin_load.sh` script.

## Copyright, License and Warranty

All work is copyright Thomas Pécseli. The project comes with no warranty, fit for particular
purpose or anything like that. Use it at your own risk. I haven't decided on license yet. If
you want to use anything from this project, then you are welcome. I don't know what license
is best suited here. I'm not a lawyer.

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

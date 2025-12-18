# TO DO
- Fix time constant in call to SynthModule simulate function
  + Should be c in RK tableau (not time step)
  + Perhaps both should be available?
  + perhaps it should be c * dt?
- Cleanup all that commented out code
- example programs
  + cli-synth. Test scale part
  + cli-test. Use INI files. cli args for making notes. wav output
  + daisy synth
  + synth with GUI and plugin- code for sequencer
- multiple StateInput structs for one simulation.
  + control interface parameter to switch
- StateInput should be trait
  + version using stack programs
  + compiled version using expressions (for pre-compiled synths)
- delay module ain't finished yet
- better waveguide modules

## TODO to take it back to daisy seed ready
- reinstate `no_std` for `synth-engine`
- remove _all_ Debug and Display from `synth-engine`


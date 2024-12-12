use std::path::Path;
use synth_designer::synth_spec::SynthSpec;

const SYNTH_SPEC_FILE: &str = "synth_spec.ini";

fn main() {
    let mut synth_spec = match SynthSpec::from_ini_file(SYNTH_SPEC_FILE) {
        Ok(s) => s,
        Err(err) => panic!("Error reading synth spec: {:?}", err),
    };

    synth_spec.allocate_state();

    let code = synth_spec.codegen().to_string();

    let out = std::env::var("OUT_DIR").unwrap();
    let out = Path::new(&out).join("synth_modules.rs");
    std::fs::write(out, code.as_bytes()).unwrap();
}

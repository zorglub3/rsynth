use proc_macro2::TokenStream;
use proc_macro2::Ident;
use crate::input_expr::Expr;
use crate::modules::ModuleSpec;
use crate::synth_spec::SynthSpec;
use quote::quote;
use quote::format_ident;
use synth_engine::stack_program::compute_stack_size;
use crate::synth_spec::quote_instruction;

pub struct Codegen {
    stack_program_code: Vec<TokenStream>,
    wavetable_code: Vec<TokenStream>,
    databuffer_code: Vec<TokenStream>,
    synthmodule_code: Vec<TokenStream>,
    stack_program_ident_counter: usize,
    wavetable_ident_counter: usize,
    databuffer_ident_counter: usize,
    synthmodule_ident_counter: usize,
    max_stack_size: usize,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            stack_program_code: Vec::new(),
            wavetable_code: Vec::new(),
            databuffer_code: Vec::new(),
            synthmodule_code: Vec::new(),
            stack_program_ident_counter: 0,
            wavetable_ident_counter: 0,
            databuffer_ident_counter: 0,
            synthmodule_ident_counter: 0,
            max_stack_size: 1,
        }
    }

    pub fn add_stack_program(&mut self, expr: &Expr, synth_spec: &SynthSpec) -> Ident {
        let c = self.stack_program_ident_counter;

        let instructions = expr.compile_to_instructions(synth_spec).unwrap();
        let stack_size = compute_stack_size(&instructions);
        let instructions_code: Vec<TokenStream> = 
            instructions.iter().map(quote_instruction).collect();

        let instructions_id = format_ident!("instructions_{}", c);
        let program_id = format_ident!("stack_program_{}", c);

        self.stack_program_ident_counter += 1;

        self.stack_program_code.push(
            quote! { 
                let #instructions_id = [ #(#instructions_code),* ];
                let #program_id = StackProgram { 
                    code: &#instructions_id, 
                    stack_size: #stack_size,
                };
            }
        );

        self.max_stack_size = self.max_stack_size.max(stack_size);

        program_id
    }

    pub fn add_wavetable_entry(&mut self, data: Vec<f32>) -> Ident {
        todo!()
    }

    pub fn add_databuffer(&mut self, size: usize) -> Ident {
        todo!()
    }

    pub fn add_synthmodule(&mut self, module_spec: &dyn ModuleSpec, synth_spec: &SynthSpec) -> Ident {
        todo!()
    }

    pub fn generate_all_code(self) -> TokenStream {
        todo!()
    }
}

use gcd::*;

use crate::gcd::generate_gcd;

pub mod display;
pub mod eval;
pub mod gcd;
pub mod typecheck;
pub mod types;

fn main() {
    let program = generate_gcd(a_b_gcd_parameter_sequence());
    match program.typecheck() {
        Ok(annotated_program) => {
            let value_sequence = vec![60.into(), 24.into(), 0.into(), 0.into()]; // gcd(60,24) = 12
            match annotated_program.eval(value_sequence.clone()) {
                Ok(result) => println!("result: {result:?} for inputs {value_sequence:?} to program:\n{program}"),
                Err(reason) => println!("exec failed: {reason} for program:\n{program}"),
            }
        }
        Err(reason) => println!("typecheck failed: {reason} for program:\n{program}"),
    }
}

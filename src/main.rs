use gcd::*;

#[allow(dead_code)]

use crate::gcd::generate_gcd;

pub mod types;
pub mod display;
pub mod typecheck;
pub mod gcd;

fn main() {
        let program = generate_gcd(a_b_gcd_parameter_sequence());
        if let Err(reason) = program.typecheck() {
            println!("typecheck failed: {reason} for program:\n{program}");
        } else {
            println!("typecheck passed for program:\n{program}");
        }
}

#[cfg(test)]
mod tests {
    use crate::gcd::{a_b_gcd_parameter_sequence, a_a_gcd_parameter_sequence};

    use super::*;

    #[test]
    fn test_typecheck_gcd() -> Result<(), String>  {
        let program = generate_gcd(a_b_gcd_parameter_sequence());
        program.typecheck()
    }

    #[test]
    fn test_typecheck_duplicate_parameters()  -> Result<(), String> {
        let program = generate_gcd(a_a_gcd_parameter_sequence());
        match program.typecheck() {
            Ok(_) => Err("should fail".into()),
            Err(reason) => {
                assert_eq!("parameter a declared twice", reason);
                Ok(())},
        }
    }

    #[test]
    fn test_typecheck_undefined_variable() -> Result<(), String> {
        let program = generate_gcd(x_y_gcd_parameter_sequence());
        match program.typecheck() {
            Ok(_) => Err("should fail".into()),
            Err(reason) => {
                assert_eq!("identifier a is not defined", reason);
                Ok(())},
        }
    }

    #[test]
    fn test_typecheck_duplicate_parameters_2() -> Result<(), String> {
        let program = generate_gcd(a_g_gcd_parameter_sequence());
        match program.typecheck() {
            Ok(_) => Err("should fail".into()),
            Err(reason) => {
                assert_eq!("function parameter g declared twice", reason);
                Ok(())},
        }
    }
}

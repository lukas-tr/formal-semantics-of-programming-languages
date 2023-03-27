#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod types;
mod display;
mod typecheck;

use types::*;

// var c:Nat;
// procedure div(a:Nat,b:Nat; ref q:Nat,r:Nat) {
// q := 0; r := a;
// while r >= b do {
// q := q+1; r := r-b
// }
// c := c+1
// }
// procedure gcd(a:Nat,b:Nat; ref g:Nat,n:Nat) {
// c := 0;
// while a > 0 ∧ b > 0 do {
// var c:Nat;
// if a ≥ b
// then call div(a,b;c,a)
// else call div(b,a;c,b)
// }
// if a > 0 then g := a else g := b;
// n := c
// }
// program main(a:Nat,b:Nat,c:Nat,d:Nat) {
// call gcd(a*b,a+b;c,d)
// }
fn generate_gcd() -> Program<'static> {
    let a_b_parameter_sequence = Parameters::Sequence(
        Parameters::Sequence(Parameters::Empty.into(), "a".into(), "Nat".into()).into(),
        "b".into(),
        "Nat".into(),
    );

    let div_while = Command::While(
        Expression::LessThanOrEqual("b".into(), "r".into()),
        Command::Sequence(
            Command::Assign("q".into(), Expression::Sum("q".into(), 1.into())).into(),
            Command::Assign("r".into(),Expression::Difference("r".into(), "b".into())).into(),
        )
        .into(),
    );

    let c = Declaration::Variable("c".into(), "Nat".into());

    let div = Declaration::Procedure(
        "div".into(),
        a_b_parameter_sequence.clone(),
        Parameters::Sequence(
            Parameters::Sequence(Parameters::Empty.into(), "q".into(), "Nat".into()).into(),
            "r".into(),
            "Nat".into(),
        ),
        Command::Sequence(
            Command::Sequence(
                Command::Assign("q".into(), 0.into()).into(),
                Command::Assign("r".into(), "a".into()).into(),
            )
            .into(),
            Command::Sequence(
                div_while.into(),
                Command::Assign("c".into(), Expression::Sum("c".into(), 1.into())).into(),
            )
            .into(),
        )
        .into(),
    );

    let while_body = Command::Var(
        "c".into(),
        "Nat".into(),
        Command::IfElse(
            Expression::LessThanOrEqual("b".into(), "a".into()).into(),
            Command::Call(
                "div".into(),
                Expressions::Sequence(
                    "a".into(),
                    Expressions::Sequence("b".into(), Expressions::Empty.into()).into(),
                )
                .into(),
                Variables::Sequence(
                    "c".into(),
                    Variables::Sequence("a".into(), Variables::Empty.into()).into(),
                )
                .into(),
            )
            .into(),
            Command::Call(
                "div".into(),
                Expressions::Sequence(
                    "b".into(),
                    Expressions::Sequence("a".into(), Expressions::Empty.into()).into(),
                )
                .into(),
                Variables::Sequence(
                    "c".into(),
                    Variables::Sequence("b".into(), Variables::Empty.into()).into(),
                )
                .into(),
            )
            .into(),
        )
        .into(),
    );

    let gcd_if = Command::IfElse(
        Expression::Not(Expression::LessThanOrEqual("a".into(), 0.into()).into()).into(),
        Command::Assign("g".into(), "a".into()).into(),
        Command::Assign("g".into(), "b".into()).into(),
    );

    let gcd = Declaration::Procedure(
        "gcd".into(),
        a_b_parameter_sequence,
        Parameters::Sequence(
            Parameters::Sequence(Parameters::Empty.into(), "c".into(), "Nat".into()).into(),
            "d".into(),
            "Nat".into(),
        ),
        Command::Sequence(
            Command::Assign("c".into(), 0.into()).into(),
            Command::Sequence(
                Command::While(
                    Expression::And(
                        Expression::Not(Expression::LessThanOrEqual("a".into(), 0.into()).into())
                            .into(),
                        Expression::Not(Expression::LessThanOrEqual("b".into(), 0.into()).into())
                            .into(),
                    ),
                    while_body.into(),
                )
                .into(),
                Command::Sequence(
                    gcd_if.into(),
                    Command::Assign("n".into(), "c".into()).into(),
                )
                .into(),
            )
            .into(),
        ),
    );

    let declarations = Declarations::Sequence(
        Declarations::Sequence(
            Declarations::Sequence(Declarations::Empty.into(), c).into(),
            div,
        )
        .into(),
        gcd,
    );
    let parameters = Parameters::Sequence(
        Parameters::Sequence(
            Parameters::Sequence(
                Parameters::Sequence(Parameters::Empty.into(), "a".into(), "Nat".into()).into(),
                "b".into(),
                "Nat".into(),
            )
            .into(),
            "c".into(),
            "Nat".into(),
        )
        .into(),
        "d".into(),
        "Nat".into(),
    );

    let main = Command::Call(
        "gcd".into(),
        Expressions::Sequence(
            Expression::Product("a".into(), "b".into()),
            Expressions::Sequence(
                Expression::Sum("a".into(), "b".into()).into(),
                Expressions::Empty.into(),
            )
            .into(),
        ),
        Variables::Sequence(
            "c".into(),
            Variables::Sequence("d".into(), Variables::Empty.into()).into(),
        ),
    );

    let program = Program(declarations, "gcd".into(), parameters, main);

    program
}

fn main() {
    // let program = Program("gcd".into(), Parameter::None, Command::None);
    // println!("{}", program.get_linear_representation());
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_add() {
        let program = generate_gcd();
        // let program = Program("gcd".into(), Parameter::None, Command::None);
        println!("{}", program);
    }

    #[test]
    fn test_typecheck() {
        let program = generate_gcd();
        let variables = HashMap::new();
        println!("{:?}", program.typecheck(&variables));
    }

    #[test]
    fn test_bad_add() {
        let b = Expression::Not(
            Expression::Equal(
                Expression::Variable("i".into()).into(),
                Expression::Variable("n".into()).into(),
            )
            .into(),
        );
        let c1 = Command::Assign(
            "j".into(),
            Expression::Sum(
                Expression::Product(
                    Expression::Value(2.into()).into(),
                    Expression::Variable("i".into()).into(),
                )
                .into(),
                Expression::Value(1.into()).into(),
            )
            .into(),
        );
        let c2 = Command::Assign(
            "a".into(),
            Expression::Sum(
                Expression::Variable("a".into()).into(),
                Expression::Variable("j".into()).into(),
            ),
        );
        let c3 = Command::Assign(
            "i".into(),
            Expression::Sum(
                Expression::Variable("i".into()).into(),
                Expression::Value(1.into()).into(),
            )
            .into(),
        );
        let c = Command::While(
            b,
            Command::Var(
                "j".into(),
                "Int".into(),
                Command::Sequence(c1.into(), Command::Sequence(c2.into(), c3.into()).into()).into(),
            )
            .into(),
        );
        // should result in something like this:
        // while (˜i=n) do {var j; {j:=((2*i)+1); {a:=(a+j); i:=(i+1)}}}

        // expected computation result:
        // n=5 a=25 i=5 (default variable value = 0)
        println!("{}", c);
    }
}

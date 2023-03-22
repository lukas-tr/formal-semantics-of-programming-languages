#![allow(dead_code, unused_variables, unused_imports)]

mod types;
use types::*;
mod display;
use display::*;
mod typecheck;
use typecheck::*;

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
let c = Declaration::Variable(Identifier("c".into()), Sort("Nat".into()));
// let div = Declaration::Procedure(
//     Identifier("div".into()),
//     Parameters::Sequence(
//         Box::new(Parameters::Sequence(
//             Box::new(Parameters::Sequence(
//                 Box::new(Parameters::Empty),
//                 Variable(Identifier("q".into()), Sort("Nat".into())),
//                 Sort("ref Nat".into()),
//             )),
//             Variable(Identifier("r".into()), Sort("Nat".into())),
//             Sort("ref Nat".into()),
//         )),
//         Variable(Identifier("a".into()), Sort("Nat".into())),
//         Sort("Nat".into()),
//         Variable(Identifier("b".into()), Sort("Nat".into())),
//         Sort("Nat".into()),
//     ),
//     Command::Sequence(
//         Command::Assign(Identifier("q".into()), Expression::Numeral(0.into()).into()).into(),
//         Command::Assign(
//             Identifier("r".into()),
//             Expression::Identifier("a".into()).into(),
//         )
//         .into(),
//         Command::While(
//             Expression::LessThanOrEqual(
//                 Expression::Identifier("r".into()).into(),
//                 Expression::Identifier("b".into()).into(),
//             )
//             .into(),
//             Command::Sequence(
//                 Command::Assign(
//                     Identifier("q".into()),
//                     Expression::Sum(
//                         Expression::Identifier("q".into()).into(),
//                         Expression::Numeral(1.into()).into(),
//                     )
//                     .into(),
//                 )
//                 .into(),
//                 Command::Assign(
//                     Identifier("r".into()),
//                     Expression::Difference(
//                         Expression::Identifier("r".into()).into(),
//                         Expression::Identifier("b".into()).into(),
//                     )
//                     .into(),
//                 )
//                 .into(),
//             )
//             .into(),
//         )
//         .into(),
//         Command::Assign(
//             Identifier("c".into()),
//             Expression::Sum(
//                 Expression::Identifier("c".into()).into(),
//                 Expression::Numeral(1.into()).into(),
//             )
//             .into(),
//         )
//         .into(),
//     )
//     .into(),
// );
// let gcd = Declaration::Procedure(
//     Identifier("gcd".into()),
//     Parameters::Sequence(
//         Box::new(Parameters::Sequence(
//             Box::new(Parameters::Sequence(
//                 Box::new(Parameters::Empty),
//                 Variable(Identifier("g".into()), Sort("Nat".into())),
//                 Sort("ref Nat".into()),
//             )),
//             Variable(Identifier("n".into()), Sort("Nat".into())),
//             Sort("Nat".into()),
//         )),
//         Variable(Identifier("a".into()), Sort("Nat".into())),
//         Sort("Nat".into()),
//         Variable(Identifier("b".into()), Sort("Nat".into())),
//         Sort("Nat".into()),
//     ),
//     Command::Sequence(
//         Command::Assign(
//             Identifier("c".into()),
//             Expression::Numeral(0.into()).into(),
//         )
//         .into(),
//         Command::While(
//             Expression::And(
//                 Expression::GreaterThan(Expression::Identifier("a".into()).into(), Expression::Numeral(0.into()).into()).into(),
//                 Expression::GreaterThan(Expression::Identifier("b".into()).into(), Expression::Num

    let declarations = Declarations::Sequence(c, Declarations::Empty.into());
    let parameters = Parameters::Empty;
    let main = Command::Sequence(Command::Call("gcd".into(), Expressions::Empty, Variables::Empty).into(), Command::Empty.into());
    let program = Program(declarations, "gcd".into(), parameters, main);

    program
}


// program gcd(a:Nat,b:Nat,g:Nat,n:Nat) {
// var c:Nat; c := 0;
// while a > 0 ∧ b > 0 do {
// if a ≥ b
// then a := a-b
// else b := b-a
// c := c+1
// }
// if a > 0
// then g := a
// else g := b;
// n

// Definition 7.1 (Program) A program is a phrase P ∈ Program which is formed
// according to the following grammar:
// P ∈ Program, X ∈ Parameters, C ∈ Command
// F ∈ Formula, T ∈ Term, V ∈ Variable, S ∈ Sort, I ∈ Identifier
// P ::= program I(X) C
// X ::= | X, V: S
// C ::= V:= T | var V: S; C | C1;C2 |
// | if F then C1 else C2 | if F then C | while F do C
// Here Parameters denotes the syntactic domain of parameters while Command
// denotes the syntactic domain of commands. The syntactic domains Formula
// of formulas, Term of terms, Variable of variables, Sort of sorts, and Identifier
// of identifiers, are formed as in Definition 6.1.

// type numeral = int ;;
// type identifier = string ;;
// type expression =
// | N of numeral
// | I of identifier
// | S of expression * expression
// | P of expression * expression
// ;;
// type boolexp =
// | Eq of expression * expression
// | Not of boolexp
// | And of boolexp * boolexp
// ;;
// type command =
// | Assign of identifier * expression
// | Var of identifier * command
// | Seq of command * command
// | If2 of boolexp * command * command
// | If1 of boolexp * command
// | While of boolexp * command
// ;;

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
                Expression::Identifier("i".into()).into(),
                Expression::Identifier("n".into()).into(),
            )
            .into(),
        );
        let c1 = Command::Assign(
            "j".into(),
            Expression::Sum(
                Expression::Product(
                    Expression::Numeral(2.into()).into(),
                    Expression::Identifier("i".into()).into(),
                )
                .into(),
                Expression::Numeral(1.into()).into(),
            )
            .into(),
        );
        let c2 = Command::Assign(
            "a".into(),
            Expression::Sum(
                Expression::Identifier("a".into()).into(),
                Expression::Identifier("j".into()).into(),
            ),
        );
        let c3 = Command::Assign(
            "i".into(),
            Expression::Sum(
                Expression::Identifier("i".into()).into(),
                Expression::Numeral(1.into()).into(),
            )
            .into(),
        );
        let c = Command::While(
            b,
            Command::Var(
                "j".into(),
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

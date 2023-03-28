use crate::types::*;

// a:Nat,b:Nat
pub fn a_b_gcd_parameter_sequence() -> Parameters<'static> {
    Parameters::Sequence(
        Parameters::Sequence(Parameters::Empty.into(), "a".into(), "Int".into()).into(),
        "b".into(),
        "Int".into(),
    )
}

// a:Nat,a:Nat
pub fn a_a_gcd_parameter_sequence() -> Parameters<'static> {
    Parameters::Sequence(
        Parameters::Sequence(Parameters::Empty.into(), "a".into(), "Int".into()).into(),
        "a".into(),
        "Int".into(),
    )
}

// x:Nat,y:Nat
pub fn x_y_gcd_parameter_sequence() -> Parameters<'static> {
    Parameters::Sequence(
        Parameters::Sequence(Parameters::Empty.into(), "x".into(), "Int".into()).into(),
        "y".into(),
        "Int".into(),
    )
}

// a:Nat,g:Nat
pub fn a_g_gcd_parameter_sequence() -> Parameters<'static> {
    Parameters::Sequence(
        Parameters::Sequence(Parameters::Empty.into(), "a".into(), "Int".into()).into(),
        "g".into(),
        "Int".into(),
    )
}

//  var c:Nat;
//  procedure div(a:Nat,b:Nat; ref q:Nat,r:Nat) {
//      q := 0; r := a;
//      while r >= b do {
//          q := q+1; r := r-b
//      }
//       c := c+1
//  }
//  procedure gcd(a:Nat,b:Nat; ref g:Nat,n:Nat) {
//      c := 0;
//      while a > 0 ∧ b > 0 do {
//          var c:Nat;
//          if a ≥ b
//          then call div(a,b;c,a)
//          else call div(b,a;c,b)
//      }
//      if a > 0 then g := a else g := b;
//      n := c
//  }
//  program main(a:Nat,b:Nat,c:Nat,d:Nat) {
//      call gcd(a*b,a+b;c,d)
//  }
pub fn generate_gcd(gcd_parameter_sequence: Parameters<'static>) -> Program<'static> {
    let div_while = Command::While(
        Expression::LessThanOrEqual("b".into(), "r".into()),
        Command::Sequence(
            Command::Assign("q".into(), Expression::Sum("q".into(), 1.into())).into(),
            Command::Assign("r".into(), Expression::Difference("r".into(), "b".into())).into(),
        )
        .into(),
    );

    let c = Declaration::Variable("c".into(), "Int".into());

    let div = Declaration::Procedure(
        "div".into(),
        Parameters::Sequence(
            Parameters::Sequence(Parameters::Empty.into(), "a".into(), "Int".into()).into(),
            "b".into(),
            "Int".into(),
        ),
        Parameters::Sequence(
            Parameters::Sequence(Parameters::Empty.into(), "q".into(), "Int".into()).into(),
            "r".into(),
            "Int".into(),
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
        "Int".into(),
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
        gcd_parameter_sequence,
        Parameters::Sequence(
            Parameters::Sequence(Parameters::Empty.into(), "g".into(), "Int".into()).into(),
            "n".into(),
            "Int".into(),
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
                Parameters::Sequence(Parameters::Empty.into(), "a".into(), "Int".into()).into(),
                "b".into(),
                "Int".into(),
            )
            .into(),
            "c".into(),
            "Int".into(),
        )
        .into(),
        "d".into(),
        "Int".into(),
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

use crate::types::*;
use std::fmt::Display;

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Expression::True => todo!(),
            // Expression::False => todo!(),
            Expression::Numeral(n) => n.fmt(f),
            Expression::Identifier(i) => i.fmt(f),
            Expression::Sum(left, right) => write!(f, "({left}+{right})"),
            Expression::Difference(_, _) => todo!(),
            Expression::Product(left, right) => write!(f, "({left}*{right})"),
            Expression::Division(_, _) => todo!(),
            Expression::Negative(_) => todo!(),
            Expression::Equal(left, right) => write!(f, "{left}={right}"),
            Expression::LessThanOrEqual(_, _) => todo!(),
            Expression::And(_, _) => todo!(),
            Expression::Or(_, _) => todo!(),
            Expression::Not(inner) => write!(f, "(!{inner})"),
        }
    }
}

impl Display for Sort<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Numeral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Variable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> Display for Declaration<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::Variable(identifier, sort) => write!(f, "var {identifier}:{sort}"),
            Declaration::Procedure(identifier, parameters, body) => todo!(),
        }
    }
}

impl<'a> Display for Parameters<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Parameters::Empty => write!(f, ""),
            Parameters::Sequence(rest, variable, sort) => {
                write!(f, "{rest}")?;
                if let Parameters::Sequence(..) = **rest {
                    write!(f, ",")?;
                }
                write!(f, "{variable}:{sort}")
            }
        }
    }
}

impl<'a> Display for Program<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Program ( declarations, body, input, name) = self;
        write!(f, "{declarations}program {name} ({input}) {{{body}}}")
    }
}

impl<'a> Display for Declarations<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Declarations::Empty => write!(f, ""),
            Declarations::Sequence(declaration, rest) => write!(f, "{declaration}\n{rest}"),
        }
    }
}

impl<'a> Display for Expressions<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expressions::Empty => write!(f, ""),
            Expressions::Sequence(expression, rest) => {
                write!(f, "{expression}")?;
                if let Expressions::Sequence(_, _) = **rest {
                    write!(f, ", ")?;
                }
                write!(f, "{rest}")
            }
        }
    }
}

impl<'a> Display for Variables<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variables::Empty => write!(f, ""),
            Variables::Sequence(variable, rest) => {
                write!(f, "{variable}")?;
                if let Variables::Sequence(_, _) = **rest {
                    write!(f, ", ")?;
                }
                write!(f, "{rest}")
            }
        }
    }
}

impl<'a> Display for Command<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Empty => write!(f, ""),
            Command::Assign(name, expression) => write!(f, "{name}:={expression}"),
            Command::Var(name, rest) => write!(f, "{{var {name}; {rest}}}"),
            Command::Sequence(first, rest) => write!(f, "{{{first}; {rest}}}"),
            Command::IfElse(_, _, _) => todo!(),
            Command::If(_, _) => todo!(),

            // while (˜i=n) do {var j; {j:=((2*i)+1); {a:=(a+j); i:=(i+1)}}}
            Command::While(cond, body) => write!(f, "while {cond} do {body}"),
            Command::Call(function, input, output) => {
                write!(f, "call {function}({input};{output})")
            }
        }
    }
}

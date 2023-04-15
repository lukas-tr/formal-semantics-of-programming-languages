use crate::types::*;
use std::fmt::Display;

impl<'a> Display for Expression<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Value(n) => n.fmt(f),
            Expression::Variable(i) => i.fmt(f),
            Expression::Sum(left, right) => write!(f, "({left}+{right})"),
            Expression::Difference(left, right) => write!(f, "({left}-{right})"),
            Expression::Product(left, right) => write!(f, "({left}*{right})"),
            Expression::Division(left, right) => write!(f, "({left}/{right})"),
            Expression::Negative(expression) => write!(f, "-({expression})"),
            Expression::Equal(left, right) => write!(f, "{left}={right}"),
            Expression::LessThanOrEqual(left, right) => write!(f, "{left}≤{right}"),
            Expression::And(left, right) => write!(f, "({left}∧{right}"),
            Expression::Or(left, right) => write!(f, "({left}∨{right})"),
            Expression::Not(expression) => write!(f, "¬({expression})"),
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Numeral(n) => write!(f, "{}", n),
            Value::True => write!(f, "true"),
            Value::False => write!(f, "false"),
        }
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
            Declaration::Variable(identifier, sort) => write!(f, "var {identifier}:{sort};"),
            Declaration::Procedure(name, input_parameters, output_parameters, body) => {
                let body = indent(format!("{body}"));
                write!(f, "procedure {name} ({input_parameters}")?;
                if let Parameters::Sequence(..) = output_parameters {
                    write!(f, "; ref {output_parameters}")?;
                }
                write!(f, ") {{\n{body}\n}}")
            }
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
        let Program(declarations, name, input, body) = self;
        let body = indent(format!("{body}"));
        write!(f, "{declarations}\nprogram {name} ({input}) {{\n{body}\n}}")
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
            Command::Assign(name, expression) => write!(f, "{name}:={expression};"),
            Command::Var(name, sort, rest) => write!(f, "var {name}:{sort};\n{rest}"),
            Command::Sequence(first, rest) => write!(f, "{first}\n{rest}"),
            Command::IfElse(condition, if_branch, else_branch) => {
                let if_branch = indent(format!("{if_branch}"));
                let else_branch = indent(format!("{else_branch}"));
                write!(f, "if ({condition}) then {{\n{if_branch}\n}} else {{\n{else_branch}\n}}")
            }
            Command::If(condition, if_branch) => {
                let if_branch = indent(format!("{if_branch}"));
                write!(f, "if ({condition}) then\n{if_branch}")
            }
            Command::While(cond, body) => {
                let body = indent(format!("{body}"));
                write!(f, "while {cond} do {{\n{body}\n}}")
            }
            Command::Call(function, input, output, _) => {
                write!(f, "call {function}({input};{output});")
            }
        }
    }
}

fn indent(s: String) -> String {
    let s: String = s.lines().map(|s| format!("  {s}\n")).collect();
    s.trim_end().into()
}

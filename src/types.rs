use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Numeral(i32),
    True,
    False,
}
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Identifier<'a>(pub &'a str);
#[derive(Debug, Clone)]
pub struct Variable<'a>(pub Identifier<'a>);
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Sort<'a>(pub Identifier<'a>);

#[derive(Debug)]
pub struct Program<'a>(
    pub Declarations<'a>,
    pub Identifier<'a>,
    pub Parameters<'a>,
    pub Command<'a>,
);

#[derive(Debug)]
pub enum Declarations<'a> {
    Empty,
    Sequence(Box<Declarations<'a>>, Declaration<'a>),
}

#[derive(Debug)]
pub enum Expressions<'a> {
    Empty,
    Sequence(Expression<'a>, Box<Expressions<'a>>),
}

#[derive(Debug)]
pub enum Variables<'a> {
    Empty,
    Sequence(Variable<'a>, Box<Variables<'a>>),
}

#[derive(Debug)]
pub enum Expression<'a> {
    // True,
    // False,
    Value(Value),
    Variable(Identifier<'a>),
    Sum(Box<Expression<'a>>, Box<Expression<'a>>),
    Difference(Box<Expression<'a>>, Box<Expression<'a>>),
    Product(Box<Expression<'a>>, Box<Expression<'a>>),
    Division(Box<Expression<'a>>, Box<Expression<'a>>),
    Negative(Box<Expression<'a>>),
    Equal(Box<Expression<'a>>, Box<Expression<'a>>),
    LessThanOrEqual(Box<Expression<'a>>, Box<Expression<'a>>),
    And(Box<Expression<'a>>, Box<Expression<'a>>),
    Or(Box<Expression<'a>>, Box<Expression<'a>>),
    Not(Box<Expression<'a>>),
}

#[derive(Debug)]
pub enum Command<'a> {
    Assign(Identifier<'a>, Expression<'a>),
    Var(Identifier<'a>, Sort<'a>, Box<Command<'a>>),
    Sequence(Box<Command<'a>>, Box<Command<'a>>),
    IfElse(Expression<'a>, Box<Command<'a>>, Box<Command<'a>>),
    If(Expression<'a>, Box<Command<'a>>),
    While(Expression<'a>, Box<Command<'a>>),
    Call(Identifier<'a>, Expressions<'a>, Variables<'a>),
}

#[derive(Debug)]
pub enum Declaration<'a> {
    Variable(Identifier<'a>, Sort<'a>),
    Procedure(Identifier<'a>, Parameters<'a>, Parameters<'a>, Command<'a>),
}

#[derive(Debug, Clone)]
pub enum Parameters<'a> {
    Empty,
    Sequence(Box<Parameters<'a>>, Variable<'a>, Sort<'a>),
}

impl<'a> From<&'a str> for Identifier<'a> {
    fn from(value: &'a str) -> Self {
        Identifier(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Numeral(value)
    }
}

impl<'a> From<&'a str> for Sort<'a> {
    fn from(value: &'a str) -> Self {
        Sort(value.into())
    }
}

impl<'a> From<&'a str> for Box<Expression<'a>> {
    fn from(value: &'a str) -> Self {
        Expression::Variable(value.into()).into()
    }
}

impl<'a> From<&'a str> for Variable<'a> {
    fn from(value: &'a str) -> Self {
        Variable(value.into())
    }
}

impl<'a> From<&'a str> for Expression<'a> {
    fn from(value: &'a str) -> Self {
        Expression::Variable(value.into())
    }
}

impl<'a> From<i32> for Expression<'a> {
    fn from(value: i32) -> Self {
        Expression::Value(value.into())
    }
}

impl<'a> From<i32> for Box<Expression<'a>> {
    fn from(value: i32) -> Self {
        Expression::Value(value.into()).into()
    }
}

struct Store<'a>{
    map: HashMap<Identifier<'a>, Value>,
    default: Value,
}

impl<'a> Store<'a> {
    fn update(&mut self, identifier: Identifier<'a>, value: Value) {
        self.map.insert(identifier, value);
    }
    fn lookup(&self, identifier: Identifier<'a>)-> &Value {
        self.map.get(&identifier).unwrap_or(&self.default)
    }
    fn init(default: Value) -> Store<'a> {
        Store {
            default,
            map: HashMap::new()
        }
    }
}

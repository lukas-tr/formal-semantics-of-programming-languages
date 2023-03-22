#[derive(Debug)]
pub struct Numeral(pub i32);
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Identifier<'a>(pub &'a str);
#[derive(Debug)]

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
    Sequence(Declaration<'a>, Box<Declarations<'a>>),
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
    Numeral(Numeral),
    Identifier(Identifier<'a>),
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
    Empty,
    Assign(Identifier<'a>, Expression<'a>),
    Var(Identifier<'a>, Box<Command<'a>>),
    Sequence(Box<Command<'a>>, Box<Command<'a>>),
    IfElse(Expression<'a>, Box<Command<'a>>, Box<Command<'a>>),
    If(Expression<'a>, Box<Command<'a>>),
    While(Expression<'a>, Box<Command<'a>>),
    Call(Identifier<'a>, Expressions<'a>, Variables<'a>),
}

#[derive(Debug)]
pub enum Declaration<'a> {
    Variable(Identifier<'a>, Sort<'a>),
    Procedure(Identifier<'a>, Parameters<'a>, Command<'a>),
}

#[derive(Debug)]
pub enum Parameters<'a> {
    Empty,
    Sequence(Box<Parameters<'a>>, Variable<'a>, Sort<'a>),
}

impl<'a> From<&'a str> for Identifier<'a> {
    fn from(value: &'a str) -> Self {
        Identifier(value)
    }
}

impl From<i32> for Numeral {
    fn from(value: i32) -> Self {
        Numeral(value)
    }
}

impl<'a> From<&'a str> for Sort<'a> {
    fn from(value: &'a str) -> Self {
        Sort(value.into())
    }
}

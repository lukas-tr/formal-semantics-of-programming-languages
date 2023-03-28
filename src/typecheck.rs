use crate::types::*;
use std::collections::{HashMap, HashSet};

static INT_SORT: Sort<'static> = Sort(Identifier("Int"));
static BOOL_SORT: Sort<'static> = Sort(Identifier("Bool"));

type VariableTypingMap<'a> = HashMap<Identifier<'a>, Sort<'a>>;
type ProcedureTypingSet<'a> = HashSet<(Identifier<'a>, (Vec<Sort<'a>>, Vec<Sort<'a>>))>;

impl<'a> Program<'a> {
    pub fn typecheck(&self) -> Result<(), String> {
        let Program(declarations, _, parameters, body) = self;
        let (mut variable_typings, procedure_typings) =
            declarations.typecheck(&HashMap::new(), &HashSet::new())?;
        let (variable_typings_1, _) = parameters.typecheck(&HashMap::new(), &Vec::new())?;
        variable_typings.extend(variable_typings_1);
        body.typecheck(&variable_typings, &procedure_typings)?;
        Ok(())
    }
}

impl<'a> Declarations<'a> {
    fn typecheck(
        &self,
        _variable_typings: &VariableTypingMap<'a>,
        _procedure_typings: &ProcedureTypingSet<'a>,
    ) -> Result<(VariableTypingMap<'a>, ProcedureTypingSet<'a>), String> {
        match self {
            Declarations::Empty => Ok((HashMap::new(), HashSet::new())),
            Declarations::Sequence(other, declaration) => {
                let (variable_typings, procedure_typings) =
                    other.typecheck(_variable_typings, _procedure_typings)?;
                let (variable_typings, procedure_typings) =
                    declaration.typecheck(&variable_typings, &procedure_typings)?;

                Ok((variable_typings, procedure_typings))
            }
        }
    }
}

impl<'a> Declaration<'a> {
    fn typecheck(
        &self,
        variable_typings: &VariableTypingMap<'a>,
        procedure_typings: &ProcedureTypingSet<'a>,
    ) -> Result<(VariableTypingMap<'a>, ProcedureTypingSet<'a>), String> {
        match self {
            Declaration::Variable(identifier, sort) => {
                let mut variable_typings = variable_typings.clone();
                variable_typings.insert(*identifier, *sort);
                Ok((variable_typings, procedure_typings.clone()))
            }
            Declaration::Procedure(identifier, in_params, out_params, body) => {
                let x1 = in_params.typecheck(&HashMap::new(), &Vec::new())?;
                let mut x2 = out_params.typecheck(&HashMap::new(), &Vec::new())?;

                for (key, value) in &x1.0 {
                    if x2.0.contains_key(&key) {
                        return Err(format!("function parameter {key} declared twice"));
                    } else {
                        x2.0.insert(*key, *value);
                    }
                }

                let mut variable_typings_3 = variable_typings.clone();
                variable_typings_3.extend(x2.0);

                body.typecheck(&variable_typings_3, procedure_typings)?;

                let mut procedure_typings = procedure_typings.clone();
                procedure_typings.insert((*identifier, (x1.1, x2.1)));

                Ok((variable_typings.clone(), procedure_typings))
            }
        }
    }
}

impl<'a> Command<'a> {
    fn typecheck(
        &self,
        variable_typings: &VariableTypingMap,
        procedure_typings: &ProcedureTypingSet,
    ) -> Result<(), String> {
        match self {
            Command::Assign(identifier, expression) => {
                if let Some(variable_sort) = variable_typings.get(identifier) {
                    let expression_sort = expression.typecheck(variable_typings)?;
                    if expression_sort != *variable_sort {
                        Err(format!("expression of type {expression_sort} can't be assigned to variable of type {variable_sort}"))
                    } else {
                        Ok(())
                    }
                } else {
                    dbg!(identifier);
                    dbg!(variable_typings);
                    Err(format!("identifier {identifier} is not defined"))
                }
            }
            Command::Var(identifier, sort, command) => {
                let mut variable_typings = variable_typings.clone();
                variable_typings.insert(*identifier, *sort);
                command.typecheck(&variable_typings, procedure_typings)
            }
            Command::Sequence(first, second) => {
                first.typecheck(variable_typings, procedure_typings)?;
                second.typecheck(variable_typings, procedure_typings)
            }
            Command::IfElse(expression, branch_if, branch_else) => {
                if expression.typecheck(variable_typings)? != BOOL_SORT {
                    Err("if requires boolean expression".into())
                } else {
                    branch_if.typecheck(variable_typings, procedure_typings)?;
                    branch_else.typecheck(variable_typings, procedure_typings)
                }
            }
            Command::While(expression, branch_if) | Command::If(expression, branch_if) => {
                if expression.typecheck(variable_typings)? != BOOL_SORT {
                    Err("if and while require boolean expressions".into())
                } else {
                    branch_if.typecheck(variable_typings, procedure_typings)
                }
            }
            Command::Call(identifier, expressions, variables) => {
                let expressions = expressions.typecheck(variable_typings)?;
                let variables = variables.typecheck(variable_typings)?;

                if procedure_typings
                    .contains(&(*identifier, (expressions.clone(), variables.clone())))
                {
                    Ok(())
                } else {
                    Err(format!(
                        "no function matches signature {identifier}({expressions:?};{variables:?})"
                    ))
                }
            }
        }
    }
}

impl<'a> Expressions<'a> {
    fn typecheck(&self, variable_typings: &VariableTypingMap<'a>) -> Result<Vec<Sort<'a>>, String> {
        match self {
            Expressions::Empty => Ok(Vec::new()),
            Expressions::Sequence(first, rest) => {
                let expression_sort = first.typecheck(variable_typings)?;
                let rest = rest.typecheck(variable_typings)?;
                let mut result = vec![expression_sort];
                result.extend(rest);
                Ok(result)
            }
        }
    }
}

impl<'a> Variables<'a> {
    fn typecheck(&self, variable_typings: &VariableTypingMap<'a>) -> Result<Vec<Sort<'a>>, String> {
        match self {
            Variables::Empty => Ok(Vec::new()),
            Variables::Sequence(first, rest) => {
                if let Some(variable_sort) = variable_typings.get(&first.0) {
                    let rest = rest.typecheck(variable_typings)?;
                    let mut result = vec![*variable_sort];
                    result.extend(rest);
                    Ok(result)
                } else {
                    Err(format!("variable {first} is not defined"))
                }
            }
        }
    }
}

impl<'a> Expression<'a> {
    fn typecheck(&self, variable_typings: &VariableTypingMap<'a>) -> Result<Sort<'a>, String> {
        match self {
            Expression::Value(literal) => match literal {
                Value::Numeral(_) => Ok(INT_SORT),
                Value::True | Value::False => Ok(BOOL_SORT),
            },
            Expression::Variable(identifier) => {
                if let Some(identifier_sort) = variable_typings.get(identifier) {
                    Ok(*identifier_sort)
                } else {
                    Err(format!("identifier {identifier} is not defined"))
                }
            }
            Expression::Sum(left, right)
            | Expression::Difference(left, right)
            | Expression::Product(left, right)
            | Expression::Division(left, right) => {
                if left.typecheck(variable_typings)? != INT_SORT {
                    return Err("expected int expression".into());
                }
                if right.typecheck(variable_typings)? != INT_SORT {
                    return Err("expected int expression".into());
                }
                Ok(INT_SORT)
            }
            Expression::Negative(expression) => {
                if expression.typecheck(variable_typings)? != INT_SORT {
                    return Err("expected int expression".into());
                }
                Ok(INT_SORT)
            }
            Expression::Equal(left, right) => {
                let left = left.typecheck(variable_typings)?;
                let right = right.typecheck(variable_typings)?;
                if left != right {
                    return Err("expression type mismatch: {left}, {right}".into());
                }
                Ok(left)
            }
            Expression::LessThanOrEqual(left, right) => {
                if left.typecheck(variable_typings)? != INT_SORT {
                    return Err("expected int expression".into());
                }
                if right.typecheck(variable_typings)? != INT_SORT {
                    return Err("expected int expression".into());
                }
                Ok(BOOL_SORT)
            }
            Expression::And(left, right) | Expression::Or(left, right) => {
                if left.typecheck(variable_typings)? != BOOL_SORT {
                    return Err("expected boolean expression".into());
                }
                if right.typecheck(variable_typings)? != BOOL_SORT {
                    return Err("expected boolean expression".into());
                }
                Ok(BOOL_SORT)
            }
            Expression::Not(expression) => {
                if expression.typecheck(variable_typings)? != BOOL_SORT {
                    return Err("expected boolean expression".into());
                }
                Ok(BOOL_SORT)
            }
        }
    }
}

impl<'a> Parameters<'a> {
    fn typecheck(
        &self,
        variable_typings: &VariableTypingMap<'a>,
        sort_sequence: &[Sort<'a>],
    ) -> Result<(VariableTypingMap<'a>, Vec<Sort<'a>>), String> {
        match self {
            Parameters::Empty => Ok((HashMap::new(), Vec::new())),
            Parameters::Sequence(rest, variable, sort) => {
                let (variable_typings, sort_sequence) =
                    rest.typecheck(variable_typings, sort_sequence)?;
                let mut sort_sequence = sort_sequence.to_owned();
                let mut variable_typings = variable_typings.clone();
                if variable_typings.contains_key(&variable.0) {
                    let variable = variable.0;
                    return Err(format!("parameter {variable} declared twice"));
                }
                sort_sequence.push(*sort);
                variable_typings.insert(variable.0, *sort);
                Ok((variable_typings, sort_sequence).to_owned())
            }
        }
    }
}

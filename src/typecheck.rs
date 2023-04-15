use crate::types::*;
use std::collections::{HashMap, HashSet};

static INT_SORT: Sort<'static> = Sort(Identifier("Int"));
static BOOL_SORT: Sort<'static> = Sort(Identifier("Bool"));

pub type Signature<'a> = (Identifier<'a>, (Vec<Sort<'a>>, Vec<Sort<'a>>));
type VariableTypingMap<'a> = HashMap<Identifier<'a>, Sort<'a>>;
type ProcedureTypingSet<'a> = HashSet<Signature<'a>>;

impl<'a> Program<'a> {
    pub fn typecheck(&self) -> Result<Program, String> {
        let Program(declarations, identifier, parameters, body) = self;
        let (mut variable_typings, procedure_typings, declarations) = declarations.typecheck()?;
        let (variable_typings_1, _) = parameters.typecheck()?;
        variable_typings.extend(variable_typings_1);
        let body = body.typecheck(&variable_typings, &procedure_typings)?;
        Ok(Program(declarations, *identifier, parameters.clone(), body))
    }
}

impl<'a> Declarations<'a> {
    fn typecheck(
        &self,
    ) -> Result<
        (
            VariableTypingMap<'a>,
            ProcedureTypingSet<'a>,
            Declarations<'a>,
        ),
        String,
    > {
        match self {
            Declarations::Empty => Ok((HashMap::new(), HashSet::new(), self.clone())),
            Declarations::Sequence(other, declaration) => {
                let (variable_typings, procedure_typings, declarations) = other.typecheck()?;
                let (variable_typings, procedure_typings, declaration) =
                    declaration.typecheck(&variable_typings, &procedure_typings)?;

                Ok((
                    variable_typings,
                    procedure_typings,
                    Declarations::Sequence(declarations.into(), declaration),
                ))
            }
        }
    }
}

impl<'a> Declaration<'a> {
    fn typecheck(
        &self,
        variable_typings: &VariableTypingMap<'a>,
        procedure_typings: &ProcedureTypingSet<'a>,
    ) -> Result<
        (
            VariableTypingMap<'a>,
            ProcedureTypingSet<'a>,
            Declaration<'a>,
        ),
        String,
    > {
        match self {
            Declaration::Variable(identifier, sort) => {
                let mut variable_typings = variable_typings.clone();
                variable_typings.insert(*identifier, *sort);
                Ok((variable_typings, procedure_typings.clone(), self.clone()))
            }
            Declaration::Procedure(identifier, in_params, out_params, body) => {
                let x1 = in_params.typecheck()?;
                let mut x2 = out_params.typecheck()?;

                for (key, value) in &x1.0 {
                    if x2.0.contains_key(key) {
                        return Err(format!("function parameter {key} declared twice"));
                    } else {
                        x2.0.insert(*key, *value);
                    }
                }

                let mut variable_typings_3 = variable_typings.clone();
                variable_typings_3.extend(x2.0);

                let body = body.typecheck(&variable_typings_3, procedure_typings)?;

                let mut procedure_typings = procedure_typings.clone();
                procedure_typings.insert((*identifier, (x1.1, x2.1)));

                Ok((
                    variable_typings.clone(),
                    procedure_typings,
                    Declaration::Procedure(
                        *identifier,
                        in_params.clone(),
                        out_params.clone(),
                        body,
                    ),
                ))
            }
        }
    }
}

impl<'a> Command<'a> {
    fn typecheck(
        &self,
        variable_typings: &VariableTypingMap<'a>,
        procedure_typings: &ProcedureTypingSet<'a>,
    ) -> Result<Command<'a>, String> {
        match self {
            Command::Assign(identifier, expression) => {
                if let Some(variable_sort) = variable_typings.get(identifier) {
                    let expression_sort = expression.typecheck(variable_typings)?;
                    if expression_sort != *variable_sort {
                        Err(format!("expression of type {expression_sort} can't be assigned to variable of type {variable_sort}"))
                    } else {
                        Ok(self.clone())
                    }
                } else {
                    Err(format!("identifier {identifier} is not defined"))
                }
            }
            Command::Var(identifier, sort, command) => {
                let mut variable_typings = variable_typings.clone();
                variable_typings.insert(*identifier, *sort);
                let command = command.typecheck(&variable_typings, procedure_typings)?;
                Ok(Command::Var(*identifier, *sort, command.into()))
            }
            Command::Sequence(first, second) => {
                let first = first.typecheck(variable_typings, procedure_typings)?;
                let second = second.typecheck(variable_typings, procedure_typings)?;
                Ok(Command::Sequence(first.into(), second.into()))
            }
            Command::IfElse(expression, branch_if, branch_else) => {
                if expression.typecheck(variable_typings)? != BOOL_SORT {
                    Err("if requires boolean expression".into())
                } else {
                    let branch_if = branch_if.typecheck(variable_typings, procedure_typings)?;
                    let branch_else = branch_else.typecheck(variable_typings, procedure_typings)?;
                    Ok(Command::IfElse(
                        expression.clone(),
                        branch_if.into(),
                        branch_else.into(),
                    ))
                }
            }
            Command::If(expression, branch_if) => {
                if expression.typecheck(variable_typings)? != BOOL_SORT {
                    Err("if requires boolean expressions".into())
                } else {
                    let branch_if = branch_if.typecheck(variable_typings, procedure_typings)?;
                    Ok(Command::If(expression.clone(), branch_if.into()))
                }
            }
            Command::While(expression, body) => {
                if expression.typecheck(variable_typings)? != BOOL_SORT {
                    Err("while requires boolean expressions".into())
                } else {
                    let body = body.typecheck(variable_typings, procedure_typings)?;
                    Ok(Command::While(expression.clone(), body.into()))
                }
            }
            Command::Call(identifier, expressions, variables, _) => {
                let expression_sorts = expressions.typecheck(variable_typings)?;
                let variable_sorts = variables.typecheck(variable_typings)?;
                let signature: Signature = (
                    *identifier,
                    (expression_sorts.clone(), variable_sorts.clone()),
                );

                if procedure_typings.contains(&signature) {
                    Ok(Command::Call(
                        *identifier,
                        expressions.clone(),
                        variables.clone(),
                        Some(signature),
                    ))
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
    pub fn typecheck(
        &self,
        variable_typings: &VariableTypingMap<'a>,
    ) -> Result<Vec<Sort<'a>>, String> {
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
    pub fn typecheck(
        &self,
        variable_typings: &VariableTypingMap<'a>,
    ) -> Result<Vec<Sort<'a>>, String> {
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
    pub fn typecheck(&self) -> Result<(VariableTypingMap<'a>, Vec<Sort<'a>>), String> {
        match self {
            Parameters::Empty => Ok((HashMap::new(), Vec::new())),
            Parameters::Sequence(rest, variable, sort) => {
                let (variable_typings, sort_sequence) = rest.typecheck()?;
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

#[cfg(test)]
mod tests {
    use crate::gcd::{
        a_a_gcd_parameter_sequence, a_b_gcd_parameter_sequence, a_g_gcd_parameter_sequence,
        generate_gcd, x_y_gcd_parameter_sequence,
    };

    #[test]
    fn test_typecheck_gcd() -> Result<(), String> {
        let program = generate_gcd(a_b_gcd_parameter_sequence());
        program.typecheck()?;
        Ok(())
    }

    #[test]
    fn test_typecheck_duplicate_parameters() -> Result<(), String> {
        let program = generate_gcd(a_a_gcd_parameter_sequence());
        match program.typecheck() {
            Ok(_) => Err("should fail".into()),
            Err(reason) => {
                assert_eq!("parameter a declared twice", reason);
                Ok(())
            }
        }
    }

    #[test]
    fn test_typecheck_undefined_variable() -> Result<(), String> {
        let program = generate_gcd(x_y_gcd_parameter_sequence());
        match program.typecheck() {
            Ok(_) => Err("should fail".into()),
            Err(reason) => {
                assert_eq!("identifier a is not defined", reason);
                Ok(())
            }
        }
    }

    #[test]
    fn test_typecheck_duplicate_parameters_2() -> Result<(), String> {
        let program = generate_gcd(a_g_gcd_parameter_sequence());
        match program.typecheck() {
            Ok(_) => Err("should fail".into()),
            Err(reason) => {
                assert_eq!("function parameter g declared twice", reason);
                Ok(())
            }
        }
    }
}

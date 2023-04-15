use std::collections::HashMap;

use crate::{typecheck::Signature, types::*};

impl<'a> Program<'a> {
    pub fn eval(&self, value_sequence: Vec<Value>) -> Result<Vec<Value>, String> {
        let Program(declarations, _, parameters, body) = self;
        let (environment, top) = declarations.eval()?;

        let n = value_sequence.len();

        let argument_sequence: Vec<usize> = (top..(top + n)).collect();

        let var = parameters.eval(&argument_sequence, &environment.0)?;
        let environment = (var, environment.1.clone());

        let mut store = Store::init(Value::Numeral(0));
        for i in 0..n {
            store = store.update(argument_sequence[i], value_sequence[i].clone())
        }

        let store = body.eval(&store, &environment, top + n)?;

        let mut value_sequence = vec![];

        for i in 0..n {
            value_sequence.push(store.lookup(&argument_sequence[i]).clone());
        }

        Ok(value_sequence)
    }
}

impl<'a> Declarations<'a> {
    fn eval(&self) -> Result<(Environment<'a>, Address), String> {
        match self {
            Declarations::Empty => Ok((
                (VariableEnvironment::init(), ProcedureEnvironment::init()),
                0,
            )),
            Declarations::Sequence(other, declaration) => {
                let (environment, top) = other.eval()?;
                let (environment, top) = declaration.eval(&environment, top)?;

                Ok((environment, top))
            }
        }
    }
}

impl<'a> Parameters<'a> {
    fn eval(
        &self,
        address_sequence: &[usize],
        environment: &VariableEnvironment<'a>,
    ) -> Result<VariableEnvironment<'a>, String> {
        match self {
            Parameters::Empty => Ok(environment.clone()),
            Parameters::Sequence(others, variable, _) => {
                if let Some((address, address_sequence)) = address_sequence.split_last() {
                    let environment = others.eval(address_sequence, environment)?;
                    let environment = environment.update(&variable.0, address);
                    Ok(environment)
                } else {
                    Err("address sequence size mismatch".to_string())
                }
            }
        }
    }
}

impl<'a> Declaration<'a> {
    fn eval(
        &self,
        environment: &Environment<'a>,
        top: Address,
    ) -> Result<(Environment<'a>, Address), String> {
        match self {
            Declaration::Variable(identifier, _) => {
                let environment = (
                    environment.0.update(identifier, &top),
                    environment.1.clone(),
                );
                let top = top + 1;
                Ok((environment, top))
            }
            Declaration::Procedure(identifier, in_params, out_params, body) => {
                let procedure = Procedure {
                    in_params: in_params.clone(),
                    out_params: out_params.clone(),
                    environment: environment.clone(),
                    body: body.clone(),
                };
                let (_, in_sorts) = in_params.typecheck()?;
                let (_, out_sorts) = out_params.typecheck()?;
                let environment = (
                    environment.0.clone(),
                    environment
                        .1
                        .update(&(*identifier, (in_sorts, out_sorts)), &procedure),
                );
                Ok((environment, top))
            }
        }
    }
}

impl<'a> Command<'a> {
    pub fn eval(
        &self,
        store: &Store,
        environment: &Environment,
        top: Address,
    ) -> Result<Store, String> {
        match self {
            Command::Assign(identifier, expression) => {
                let address = environment.0.lookup(*identifier)?;
                Ok(store.update(address, expression.eval(store, environment)?))
            }
            Command::Var(identifier, _, command) => {
                let environment = (
                    environment.0.update(identifier, &top),
                    environment.1.clone(),
                );
                let top = top + 1;
                command.eval(store, &environment, top)
            }
            Command::Sequence(first, second) => {
                second.eval(&first.eval(store, environment, top)?, environment, top)
            }
            Command::IfElse(expression, if_branch, else_branch) => {
                if expression.eval(store, environment)? == Value::True {
                    if_branch.eval(store, environment, top)
                } else {
                    else_branch.eval(store, environment, top)
                }
            }
            Command::If(expression, if_branch) => {
                if expression.eval(store, environment)? == Value::True {
                    if_branch.eval(store, environment, top)
                } else {
                    Ok(store.clone())
                }
            }
            Command::While(expression, body) => {
                fn w(
                    expression: &Expression,
                    body: &Command,
                    store: &Store,
                    environment: &Environment,
                    top: Address,
                ) -> Result<Store, String> {
                    if expression.eval(store, environment)? == Value::True {
                        let store = body.eval(store, environment, top)?;
                        w(expression, body, &store, environment, top)
                    } else {
                        Ok(store.clone())
                    }
                }
                w(expression, body, store, environment, top)
            }
            Command::Call(_, expressions, variables, signature) => {
                if let Some(signature) = signature {
                    let vs = expressions.eval(store, environment)?;
                    let n = vs.len();
                    let mut store = store.clone();
                    let mut as1 = vec![];
                    for i in 0..n {
                        as1.push(top + i);
                        store = store.update(top + i, vs[i].clone());
                    }
                    let as2 = variables.eval(environment)?;

                    let procedure = environment.1.lookup(signature)?;
                    procedure.call(&as1, &as2, top + n, &store)
                } else {
                    Err("call hasn't been annotated".to_string())
                }
            }
        }
    }
}

impl<'a> Variables<'a> {
    fn eval(&self, environment: &Environment) -> Result<Vec<Address>, String> {
        match self {
            Variables::Empty => Ok(Vec::new()),
            Variables::Sequence(first, rest) => {
                let value = environment.0.lookup(first.0)?;
                let rest = rest.eval(environment)?;
                let mut result = vec![value];
                result.extend(rest);
                Ok(result)
            }
        }
    }
}

fn extract_numerals(
    store: &Store,
    environment: &Environment,
    left: &Expression,
    right: &Expression,
) -> Result<(i32, i32), String> {
    if let Value::Numeral(left) = left.eval(store, environment)? {
        if let Value::Numeral(right) = right.eval(store, environment)? {
            Ok((left, right))
        } else {
            Err("right value of the expression is not a number".into())
        }
    } else {
        Err("left value of the expression is not a number".into())
    }
}

impl<'a> Expressions<'a> {
    pub fn eval(&self, store: &Store, environment: &Environment) -> Result<Vec<Value>, String> {
        match self {
            Expressions::Empty => Ok(Vec::new()),
            Expressions::Sequence(first, rest) => {
                let value = first.eval(store, environment)?;
                let rest = rest.eval(store, environment)?;
                let mut result = vec![value];
                result.extend(rest);
                Ok(result)
            }
        }
    }
}

impl<'a> Expression<'a> {
    pub fn eval(&self, store: &Store, environment: &Environment) -> Result<Value, String> {
        match self {
            Expression::Value(val) => Ok(val.clone()),
            Expression::Variable(identifier) => {
                Ok(store.lookup(&environment.0.lookup(*identifier)?).clone())
            }
            Expression::Sum(left, right) => {
                let (left, right) = extract_numerals(store, environment, left, right)?;
                Ok(Value::Numeral(left + right))
            }
            Expression::Difference(left, right) => {
                let (left, right) = extract_numerals(store, environment, left, right)?;
                Ok(Value::Numeral(left - right))
            }
            Expression::Product(left, right) => {
                let (left, right) = extract_numerals(store, environment, left, right)?;
                Ok(Value::Numeral(left * right))
            }
            Expression::Division(left, right) => {
                let (left, right) = extract_numerals(store, environment, left, right)?;
                Ok(Value::Numeral(
                    left.checked_div(right)
                        .ok_or_else(|| "division by 0".to_string())?,
                ))
            }
            Expression::Negative(expr) => {
                if let Value::Numeral(num) = expr.eval(store, environment)? {
                    Ok(Value::Numeral(-num))
                } else {
                    Err("can only negate numbers".into())
                }
            }
            Expression::Equal(left, right) => {
                let left = left.eval(store, environment)?;
                let right = right.eval(store, environment)?;
                if left == right {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            Expression::LessThanOrEqual(left, right) => {
                let (left, right) = extract_numerals(store, environment, left, right)?;
                if left <= right {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            Expression::And(left, right) => {
                if let Value::True = left.eval(store, environment)? {
                    if let Value::True = right.eval(store, environment)? {
                        Ok(Value::True)
                    } else {
                        Ok(Value::False)
                    }
                } else {
                    Ok(Value::False)
                }
            }
            Expression::Or(left, right) => {
                if let Value::True = left.eval(store, environment)? {
                    Ok(Value::True)
                } else if let Value::True = right.eval(store, environment)? {
                    Ok(Value::True)
                } else {
                    Ok(Value::False)
                }
            }
            Expression::Not(expr) => {
                if let Value::True = expr.eval(store, environment)? {
                    Ok(Value::False)
                } else {
                    Ok(Value::True)
                }
            }
        }
    }
}

type Address = usize;

#[derive(Debug, Clone)]
pub struct Store {
    map: HashMap<Address, Value>,
    default: Value,
}

impl Store {
    pub fn update(&self, address: Address, value: Value) -> Store {
        let mut clone = self.clone();
        clone.map.insert(address, value);
        clone
    }
    pub fn lookup(&self, address: &Address) -> &Value {
        self.map.get(address).unwrap_or(&self.default)
    }
    pub fn init(default: Value) -> Store {
        Store {
            default,
            map: HashMap::new(),
        }
    }
}

type Environment<'a> = (VariableEnvironment<'a>, ProcedureEnvironment<'a>);

#[derive(Debug, Clone)]
pub struct VariableEnvironment<'a> {
    identifiers_to_addresses: HashMap<Identifier<'a>, Address>,
}

impl<'a> VariableEnvironment<'a> {
    pub fn update(
        &self,
        identifier: &Identifier<'a>,
        address: &Address,
    ) -> VariableEnvironment<'a> {
        let mut clone = self.clone();
        clone.identifiers_to_addresses.insert(*identifier, *address);
        clone
    }
    pub fn lookup(&self, identifier: Identifier) -> Result<Address, String> {
        self.identifiers_to_addresses
            .get(&identifier)
            .ok_or(format!("unknown identifier {identifier}"))
            .copied()
    }
    pub fn init() -> VariableEnvironment<'a> {
        VariableEnvironment {
            identifiers_to_addresses: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct ProcedureEnvironment<'a> {
    procedures: HashMap<Signature<'a>, Procedure<'a>>,
}

impl<'a> ProcedureEnvironment<'a> {
    pub fn update(
        &self,
        signature: &Signature<'a>,
        procedure: &Procedure<'a>,
    ) -> ProcedureEnvironment<'a> {
        let mut clone = self.clone();
        clone
            .procedures
            .insert(signature.clone(), procedure.clone());
        clone
    }
    pub fn lookup(&self, signature: &Signature<'a>) -> Result<Procedure, String> {
        self.procedures
            .get(signature)
            .ok_or(format!("unknown signature {signature:?}"))
            .cloned()
    }
    pub fn init() -> ProcedureEnvironment<'a> {
        ProcedureEnvironment {
            procedures: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct Procedure<'a> {
    in_params: Parameters<'a>,
    out_params: Parameters<'a>,
    environment: Environment<'a>,
    body: Command<'a>,
}

impl<'a> Procedure<'a> {
    fn call(
        &self,
        address_sequence_in: &[Address],
        address_sequence_out: &[Address],
        top: Address,
        store: &Store,
    ) -> Result<Store, String> {
        let var = self
            .in_params
            .eval(address_sequence_in, &self.environment.0)?;
        let var = self.out_params.eval(address_sequence_out, &var)?;

        self.body
            .eval(store, &(var, self.environment.1.clone()), top)
    }
}

#[cfg(test)]
mod tests {
    use crate::gcd::{a_b_gcd_parameter_sequence, generate_gcd};

    use super::*;

    #[test]
    fn test_gcd_60_12() -> Result<(), String> {
        let program = generate_gcd(a_b_gcd_parameter_sequence());
        let annotated_program = program.typecheck()?;

        let input_sequence = vec![60.into(), 12.into(), 0.into(), 0.into()];
        let result_sequence = annotated_program.eval(input_sequence)?;
        assert_eq!(
            result_sequence,
            vec![60.into(), 12.into(), 72.into(), 1.into()]
        );
        // gcd(60 * 12, 60 + 12) = 72 (1 iteration)

        Ok(())
    }

    #[test]
    fn test_gcd_24_60() -> Result<(), String> {
        let program = generate_gcd(a_b_gcd_parameter_sequence());
        let annotated_program = program.typecheck()?;

        let input_sequence = vec![24.into(), 60.into(), 0.into(), 0.into()];
        let result_sequence = annotated_program.eval(input_sequence)?;
        assert_eq!(
            result_sequence,
            vec![24.into(), 60.into(), 12.into(), 2.into()]
        );
        // gcd(24 * 60, 24 + 60) = 12 (2 iterations)

        Ok(())
    }

    #[test]
    fn test_gcd_6_4() -> Result<(), String> {
        let program = generate_gcd(a_b_gcd_parameter_sequence());
        let annotated_program = program.typecheck()?;

        let input_sequence = vec![6.into(), 4.into(), 0.into(), 0.into()];
        let result_sequence = annotated_program.eval(input_sequence)?;
        assert_eq!(
            result_sequence,
            vec![6.into(), 4.into(), 2.into(), 3.into()]
        );
        // gcd(6 * 4, 6 + 4) = 2 (3 iterations)

        Ok(())
    }

    #[test]
    fn test_gcd_11_13() -> Result<(), String> {
        let program = generate_gcd(a_b_gcd_parameter_sequence());
        let annotated_program = program.typecheck()?;

        let input_sequence = vec![11.into(), 13.into(), 0.into(), 0.into()];
        let result_sequence = annotated_program.eval(input_sequence)?;
        assert_eq!(
            result_sequence,
            vec![11.into(), 13.into(), 1.into(), 3.into()]
        );
        // gcd(11 * 13, 11 + 13) = 1 (3 iterations)

        Ok(())
    }

    #[test]
    fn test_eval_expression() -> Result<(), String> {
        let exp = Expression::Sum(Expression::Product(2.into(), 3.into()).into(), 4.into());
        assert_eq!(
            Value::Numeral(10),
            exp.eval(
                &Store::init(Value::Numeral(0)),
                &(VariableEnvironment::init(), ProcedureEnvironment::init())
            )?
        );
        Ok(())
    }
}

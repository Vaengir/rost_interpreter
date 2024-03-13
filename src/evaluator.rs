use crate::{
    ast::{
        BlockStatement, Expression, ExpressionStatement, Identifier, IfExpression, LetStatement,
        Program, ReturnStatement, Statement,
    },
    object::{Boolean, Environment, Integer, Object, ObjectTrait, ReturnValue},
};
use std::fmt::Display;

const NULL: Object = Object::Null;
const TRUE: Object = Object::Boolean(Boolean { value: true });
const FALSE: Object = Object::Boolean(Boolean { value: false });

fn native_bool_to_bool_struct(input: bool) -> Object {
    if input {
        return TRUE;
    }
    FALSE
}

pub trait Eval {
    fn on_eval(&self, env: &mut Environment) -> Result<Object, EvaluationError>;
}

impl Eval for Program {
    fn on_eval(&self, env: &mut Environment) -> Result<Object, EvaluationError> {
        eval_program(self, env)
    }
}

impl Eval for ExpressionStatement {
    fn on_eval(&self, env: &mut Environment) -> Result<Object, EvaluationError> {
        eval(self.expression.clone(), env)
    }
}

impl Eval for Expression {
    fn on_eval(&self, env: &mut Environment) -> Result<Object, EvaluationError> {
        match self {
            Expression::IntegerLiteral(i) => Ok(Object::Integer(Integer { value: i.value })),
            Expression::Boolean(b) => Ok(native_bool_to_bool_struct(b.value)),
            Expression::PrefixExpression(p) => {
                let right = eval(*p.right.clone(), env);
                Ok(eval_prefix_expression(&p.operator, right?)?)
            }
            Expression::InfixExpression(i) => {
                let left = eval(*i.left.clone(), env);
                let right = eval(*i.right.clone(), env);
                Ok(eval_infix_expression(&i.operator, left?, right?)?)
            }
            Expression::BlockStatement(b) => Ok(eval_block_statement(b, env)?),
            Expression::IfExpression(i) => Ok(eval_if_expression(i, env)?),
            Expression::Identifier(i) => Ok(eval_identifier(i, env)?),
            e => Err(EvaluationError::MatchError(format!(
                "Missing implementation of eval on expression: {}",
                e
            ))),
        }
    }
}

impl Eval for Statement {
    fn on_eval(&self, env: &mut Environment) -> Result<Object, EvaluationError> {
        match self {
            Statement::Let(ls) => {
                let obj = eval(ls.clone(), env)?;
                Ok(obj)
            }
            Statement::Return(rs) => {
                let obj = eval(rs.clone(), env)?;
                Ok(obj)
            }
            Statement::Expression(es) => {
                let obj = eval(es.clone(), env)?;
                Ok(obj)
            }
        }
    }
}

impl Eval for LetStatement {
    fn on_eval(&self, env: &mut Environment) -> Result<Object, EvaluationError> {
        let val = eval(self.value.clone(), env)?;
        Ok(env.set(&self.name.value, val))
    }
}

impl Eval for ReturnStatement {
    fn on_eval(&self, env: &mut Environment) -> Result<Object, EvaluationError> {
        let value = eval(self.return_value.clone(), env)?;
        Ok(Object::ReturnValue(ReturnValue {
            value: Box::new(value),
        }))
    }
}

impl Eval for BlockStatement {
    fn on_eval(&self, env: &mut Environment) -> Result<Object, EvaluationError> {
        eval_block_statement(self, env)
    }
}

impl Eval for Identifier {
    fn on_eval(&self, env: &mut Environment) -> Result<Object, EvaluationError> {
        eval_identifier(self, env)
    }
}

pub fn eval<T: Eval + std::fmt::Debug>(
    node: T,
    env: &mut Environment,
) -> Result<Object, EvaluationError> {
    node.on_eval(env)
}

fn eval_program(program: &Program, env: &mut Environment) -> Result<Object, EvaluationError> {
    let mut result: Object = NULL;
    for statement in &program.statements {
        result = eval(statement.clone(), env)?;
        if let Object::ReturnValue(rv) = result {
            return Ok(*rv.value);
        }
    }
    Ok(result)
}

fn eval_prefix_expression(operator: &str, right: Object) -> Result<Object, EvaluationError> {
    match operator {
        "!" => Ok(eval_bang_operator_expression(right)),
        "-" => Ok(eval_minus_prefix_operator_expression(right)?),
        _ => Err(EvaluationError::OperatorError(format!(
            "{}{}",
            operator,
            right.r#type()
        ))),
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        _ => FALSE,
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Result<Object, EvaluationError> {
    let integer = match right {
        Object::Integer(i) => i,
        _ => {
            return Err(EvaluationError::OperatorError(format!(
                "-{}",
                right.r#type()
            )))
        }
    };
    Ok(Object::Integer(Integer {
        value: -integer.value,
    }))
}

fn eval_infix_expression(
    operator: &str,
    left: Object,
    right: Object,
) -> Result<Object, EvaluationError> {
    if left.r#type() != right.r#type() {
        return Err(EvaluationError::TypeError(format!(
            "{} {} {}",
            left.r#type(),
            operator,
            right.r#type()
        )));
    }
    match (&left, &right) {
        (Object::Integer(i), Object::Integer(i2)) => Ok(eval_integer_infix_expression(
            operator,
            i.clone(),
            i2.clone(),
        )?),
        _ => match operator {
            "==" => Ok(native_bool_to_bool_struct(left == right)),
            "!=" => Ok(native_bool_to_bool_struct(left != right)),
            _ => Err(EvaluationError::OperatorError(format!(
                "{} {} {}",
                left.r#type(),
                operator,
                right.r#type()
            ))),
        },
    }
}

fn eval_integer_infix_expression(
    operator: &str,
    left: Integer,
    right: Integer,
) -> Result<Object, EvaluationError> {
    match operator {
        "+" => Ok(Object::Integer(Integer {
            value: left.value + right.value,
        })),
        "-" => Ok(Object::Integer(Integer {
            value: left.value - right.value,
        })),
        "*" => Ok(Object::Integer(Integer {
            value: left.value * right.value,
        })),
        "/" => Ok(Object::Integer(Integer {
            value: left.value / right.value,
        })),
        "<" => Ok(native_bool_to_bool_struct(left.value < right.value)),
        ">" => Ok(native_bool_to_bool_struct(left.value > right.value)),
        "==" => Ok(native_bool_to_bool_struct(left.value == right.value)),
        "!=" => Ok(native_bool_to_bool_struct(left.value != right.value)),
        _ => Err(EvaluationError::OperatorError(format!(
            "{} {} {}",
            left.r#type(),
            operator,
            right.r#type()
        ))),
    }
}

fn eval_if_expression(
    if_expression: &IfExpression,
    env: &mut Environment,
) -> Result<Object, EvaluationError> {
    let condition = eval(*if_expression.condition.clone(), env)?;
    if is_truthy(condition) {
        return eval(if_expression.consequence.clone(), env);
    } else if if_expression.alternative.is_some() {
        return eval(if_expression.alternative.clone().unwrap(), env);
    }
    Ok(NULL)
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Null => false,
        TRUE => true,
        FALSE => false,
        _ => true,
    }
}

fn eval_block_statement(
    block: &BlockStatement,
    env: &mut Environment,
) -> Result<Object, EvaluationError> {
    let mut result = Object::Null;
    for statement in &block.statements {
        result = eval(statement.clone(), env)?;
        if let Object::ReturnValue(_) = result {
            return Ok(result);
        }
    }
    Ok(result)
}

fn eval_identifier(node: &Identifier, env: &mut Environment) -> Result<Object, EvaluationError> {
    env.get(&node.value)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationError {
    OperatorError(String),
    TypeError(String),
    IdentError(String),
    MatchError(String),
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::OperatorError(o) => write!(f, "Unknown operator: {}", o),
            EvaluationError::TypeError(t) => write!(f, "Type mismatch: {}", t),
            EvaluationError::IdentError(i) => write!(f, "Identifier not found: {}", i),
            EvaluationError::MatchError(m) => write!(f, "MatchError: {}", m),
        }
    }
}

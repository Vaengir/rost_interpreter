use crate::{
    ast::{Expression, ExpressionStatement, LetStatement, Program, ReturnStatement, Statement},
    object::{Boolean, Integer, Object},
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
    fn on_eval(&self) -> Result<Object, EvaluationError>;
}

impl Eval for Program {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        eval_statements(&self.statements)
    }
}

impl Eval for ExpressionStatement {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        eval(self.expression.clone())
    }
}

impl Eval for Expression {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        match self {
            Expression::IntegerLiteral(i) => Ok(Object::Integer(Integer { value: i.value })),
            Expression::Boolean(b) => Ok(native_bool_to_bool_struct(b.value)),
            Expression::PrefixExpression(p) => {
                let right = eval(*p.right.clone());
                Ok(eval_prefix_expression(&p.operator, right?))
            }
            e => Err(EvaluationError::MatchError(format!(
                "Not yet implemented: {}",
                e
            ))),
        }
    }
}

impl Eval for Statement {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        match self {
            Statement::Let(ls) => {
                let obj = eval(ls.clone())?;
                Ok(obj)
            }
            Statement::Return(rs) => {
                let obj = eval(rs.clone())?;
                Ok(obj)
            }
            Statement::Expression(es) => {
                let obj = eval(es.clone())?;
                Ok(obj)
            }
        }
    }
}

impl Eval for LetStatement {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        todo!()
    }
}

impl Eval for ReturnStatement {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        todo!()
    }
}

pub fn eval<T: Eval + std::fmt::Debug>(node: T) -> Result<Object, EvaluationError> {
    Ok(node.on_eval()?)
}

fn eval_statements(statements: &[Statement]) -> Result<Object, EvaluationError> {
    let mut result: Object = NULL;
    for statement in statements {
        result = eval(statement.clone())?;
    }
    Ok(result)
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => NULL,
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

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    let integer = match right {
        Object::Integer(i) => i,
        _ => return NULL,
    };
    Object::Integer(Integer {
        value: -integer.value,
    })
}

#[derive(Debug, Clone)]
pub enum EvaluationError {
    EvalError(String),
    MatchError(String),
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::EvalError(e) => write!(f, "EvaluationError: {}", e),
            EvaluationError::MatchError(m) => write!(f, "MatchError: {}", m),
        }
    }
}
use anyhow::{anyhow, Result};
use sqlparser::ast::{
    Expr as SqlExpr,
    Value as SqlValue,
    BinaryOperator as SqlBinaryOperator,
    Statement,
    Select,
};
use std::convert::TryFrom;
use polars::prelude::*;

pub struct Sql<'a> {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by : Vec<(String, bool)>,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<usize>,
}

impl<'a> TryFrom<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_from(sql: &'a Statement) -> Result<Self, Self::Error> {
        match sql {
            // Statement::Query(q) => {
            //     println!("{:?}", q);
            //     //todo impl convert to Sql
            //     Ok(())
            // },
            _ => Err(anyhow!("only support query now")),
        }
    }
}

pub struct Expression(pub(crate) Box<SqlExpr>);
pub struct Operation(pub(crate) SqlBinaryOperator);
pub struct Value(pub(crate) SqlValue);


impl TryFrom<Expression> for Expr {
    type Error = anyhow::Error;

    fn try_from(expr: Expression) -> Result<Self, Self::Error> {
        match *expr.0 {
            SqlExpr::BinaryOp{left, op, right} => Ok(Expr::BinaryExpr{
                left: Box::new(Expression(left).try_into()?),
                op: Operation(op).try_into()?,
                right: Box::new(Expression(right).try_into()?),
            }),
            SqlExpr::Wildcard => Ok(Self::Wildcard),
            SqlExpr::IsNull(expr) => Ok(Self::IsNull(Box::new(Expression(expr).try_into()?))),
            SqlExpr::IsNotNull(expr) => Ok(Self::IsNotNull(Box::new(Expression(expr).try_into()?))),
            SqlExpr::Identifier(id) => Ok(Self::Column(Arc::new(id.value))),
            SqlExpr::Value(v) => Ok(Self::Literal(Value(v).try_into()?)),
            v => Err(anyhow!("expression: {:#?} is not supported", v)),
        }
    }
}

impl TryFrom<Operation> for Operator {
    type Error = anyhow::Error;

    fn try_from(op: Operation) -> Result<Self, Self::Error> {
        match op.0 {
            SqlBinaryOperator::Plus => Ok(Self::Plus),
            SqlBinaryOperator::Minus => Ok(Self::Minus),
            SqlBinaryOperator::Multiply => Ok(Self::Multiply),
            SqlBinaryOperator::Divide => Ok(Self::Divide),
            SqlBinaryOperator::Modulo => Ok(Self::Modulus),
            SqlBinaryOperator::Gt => Ok(Self::Gt),
            SqlBinaryOperator::Lt => Ok(Self::Lt),
            SqlBinaryOperator::GtEq => Ok(Self::GtEq),
            SqlBinaryOperator::LtEq => Ok(Self::LtEq),
            SqlBinaryOperator::Eq => Ok(Self::Eq),
            SqlBinaryOperator::NotEq => Ok(Self::NotEq),
            SqlBinaryOperator::And => Ok(Self::And),
            SqlBinaryOperator::Or => Ok(Self::Or),
            v => Err(anyhow!("Operator: {} is not suppported", v))
        }
    }
} 

impl TryFrom<Value> for LiteralValue {
    type Error = anyhow::Error;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.0 {
            SqlValue::Number(v, _) => Ok(LiteralValue::Float64(v.parse().unwrap())),
            SqlValue::Boolean(v) => Ok(LiteralValue::Boolean(v)),
            SqlValue::Null => Ok(LiteralValue::Null),
            v => Err(anyhow!("Value: {} is not supported", v))
        }
    }
}
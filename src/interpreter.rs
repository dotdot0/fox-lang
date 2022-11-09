#![allow(unused)]
use std::borrow::Borrow;

use crate::{expr::ExprVisitor, token::Object, ast::{Stmt, Expr}, error::LoxError};

struct Interpreter{}

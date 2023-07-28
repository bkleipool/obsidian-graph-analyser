//! This module helps with evaluating filtering expressions
//! 
//! - `(A & tag:#B) | (C & -tag:#D)` -> expression (describes boolean logic operations)
//! - `tag:#B` -> filter (describes specific field which is filtered)
//! - `B` -> query (regex which returns true / false)

use crate::Page;

/// Defines the boolean expression tree data structure
#[derive(Debug)]
pub enum BooleanExpr {
    Not(Box<BooleanExpr>),
    And(Box<BooleanExpr>, Box<BooleanExpr>),
    Or(Box<BooleanExpr>, Box<BooleanExpr>),
    Filter(String),
}

/// Describes which parsing error occured in [parse_boolean_expr]
#[derive(Debug)]
pub enum ParsingError {
    UnmatchedParentheses,
    MissingOperand(String),
    MissingOperator(String),
    InvalidExpression(String),
}

// Helper function to check if a character is a valid query character
fn is_valid_char(c: char) -> bool {
    c != '(' && c != ')' && c != '&' && c != '|' && c != '-' && c != ' '
}

/// Turns a string into a boolean syntax tree (recursively)
/// - `&` = AND operator
/// - `|` = OR operator
/// - `-` = NOT operator
pub fn parse_boolean_expr(expr: &str) -> Result<BooleanExpr, ParsingError> {
    let expr = expr.trim();

    // Check for the base case (single variable)
    if expr.chars().all(is_valid_char) {
        return Ok(BooleanExpr::Filter(expr.to_string()));
    }

    // Check for NOT operator
    if expr.starts_with('-') {
        let inner_expr = parse_boolean_expr(&expr[1..])?;
        return Ok(BooleanExpr::Not(Box::new(inner_expr)));
    }

    // Find the index where the AND or OR operator is located
    let mut idx = expr.len() - 1;
    let mut level = 0;

    while idx > 0 {
        let c = expr.chars().nth(idx).unwrap();
        match c {
            '(' => level -= 1,
            ')' => level += 1,
            '&' if level == 0 => break,
            '|' if level == 0 => break,
            _ => {}
        }
        idx -= 1;
    }

    if idx == 0 {
        // The expression is surrounded by parentheses, so we ignore them and parse the inner part
        if expr.starts_with('(') && expr.ends_with(')') {
            return parse_boolean_expr(&expr[1..expr.len() - 1]);
        }
        // Invalid expression
        if level != 0 {
            return Err(ParsingError::UnmatchedParentheses);
        } else if expr.chars().all(is_valid_char) {
            return Err(ParsingError::MissingOperator(expr.to_string()));
        } else {
            return Err(ParsingError::InvalidExpression(expr.to_string()));
        }
    }

    let left_expr = parse_boolean_expr(&expr[..idx])?;
    let right_expr = parse_boolean_expr(&expr[idx + 1..])?;

    match expr.chars().nth(idx).unwrap() {
        '&' => Ok(BooleanExpr::And(Box::new(left_expr), Box::new(right_expr))),
        '|' => Ok(BooleanExpr::Or(Box::new(left_expr), Box::new(right_expr))),
        _ => Err(ParsingError::InvalidExpression("Bruh2".to_string())),
    }
}

/// Evaluates a (nested) boolean expression for some input [Page]
pub fn evaluate_expr(expr: &BooleanExpr, input: &Page) -> bool {
    match expr {
        BooleanExpr::Not(inner_expr) => !evaluate_expr(inner_expr.as_ref(), input),
        BooleanExpr::And(inner_expr_left, inner_expr_right) => {
            evaluate_expr(inner_expr_left.as_ref(), input)
                && evaluate_expr(inner_expr_right.as_ref(), input)
        }
        BooleanExpr::Or(inner_expr_left, inner_expr_right) => {
            evaluate_expr(inner_expr_left.as_ref(), input)
                || evaluate_expr(inner_expr_right.as_ref(), input)
        }
        BooleanExpr::Filter(filter_string) => evaluate_filter(filter_string, input),
    }
}

/// Evaluates a filter for some input [Page]
pub fn evaluate_filter(filter: &str, input: &Page) -> bool {
    // Page filter
    if filter.starts_with("title:") {
        let query = filter.strip_prefix("title:").unwrap();
        input.title.contains(query)

    // Tag filter
    } else if filter.starts_with("tag:#") {
        let query = filter.strip_prefix("tag:#").unwrap();

        input.tags.iter().any(|tag| tag.contains(query))

    // Page filter
    } else {
        let query = filter;
        input.title.contains(query)
    }
}

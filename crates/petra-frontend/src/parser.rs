use pest::Parser;
use pest_derive::Parser;

use petra_core::{Document, TopItem, VarDeclaration, VarValue};
use std::io::Read;
use std::num::ParseIntError;
use thiserror::Error;
use tracing::error;

#[derive(Parser)]
#[grammar = "petra.pest"]
struct PetraParser;
const STRING_VAR_TYPE: &str = "string";
const I64_VAR_TYPE: &str = "i64";

pub fn parse<R: Read>(mut reader: R) -> Result<Document, PetraFrontendError> {
    let mut str_input = String::new();
    reader
        .read_to_string(&mut str_input)
        .map_err(PetraFrontendError::InputError)?;
    let pairs = PetraParser::parse(Rule::program, &str_input).map_err(Box::new)?;
    let mut res: Document = Document::new();
    for pair in pairs {
        let item = match pair.as_rule() {
            Rule::var_declaration => Some(TopItem::VarDeclaration(parse_var_declaration(
                pair.into_inner(),
            )?)),
            Rule::new_line_comment => Some(TopItem::Comment(
                get_solo_item(pair, Rule::new_line_comment_val)?
                    .as_str()
                    .to_string(),
            )),
            Rule::multi_line_comment => Some(TopItem::MultiLineComment(
                get_solo_item(pair, Rule::multi_line_comment_val)?
                    .as_str()
                    .to_string(),
            )),
            _ => None,
        };
        if let Some(item) = item {
            res.items.push(item);
        }
    }

    Ok(res)
}
fn parse_var_declaration(
    mut pairs: pest::iterators::Pairs<Rule>,
) -> Result<VarDeclaration, PetraFrontendError> {
    let name = verify_type(&mut pairs, Rule::var_name)?
        .as_str()
        .to_string();
    let var_type = verify_type(&mut pairs, Rule::var_type)?.as_str();
    let var_value = verify_type(&mut pairs, Rule::var_value)?;
    force_empty(&mut pairs)?;
    Ok(VarDeclaration::new(
        name,
        create_value(var_type, var_value)?,
    ))
}

fn verify_type<'a>(
    pairs: &mut pest::iterators::Pairs<'a, Rule>,
    rule_type: Rule,
) -> Result<pest::iterators::Pair<'a, Rule>, PetraFrontendError> {
    let pair = next_item(pairs)?;
    if pair.as_rule() == rule_type {
        Ok(pair)
    } else {
        Err(PetraFrontendError::ProcessError(format!(
            "got wrong rule type, expected={:?}, actual={:?}",
            rule_type,
            pair.as_rule()
        )))
    }
}
fn get_solo_item(
    pair: pest::iterators::Pair<'_, Rule>,
    rule_type: Rule,
) -> Result<pest::iterators::Pair<'_, Rule>, PetraFrontendError> {
    let mut pairs = pair.into_inner();
    let res = verify_type(&mut pairs, rule_type)?;
    force_empty(&mut pairs)?;
    Ok(res)
}
fn next_item<T>(mut iter: impl Iterator<Item = T>) -> Result<T, PetraFrontendError> {
    iter.next()
        .ok_or_else(|| PetraFrontendError::ProcessError("Not enough items to process".into()))
}
fn force_empty<T>(mut iter: impl Iterator<Item = T>) -> Result<(), PetraFrontendError> {
    match iter.next() {
        Some(_) => Err(PetraFrontendError::ProcessError(
            "too much items to process".into(),
        )),
        None => Ok(()),
    }
}
fn create_value(
    var_type: &str,
    var_value: pest::iterators::Pair<'_, Rule>,
) -> Result<VarValue, PetraFrontendError> {
    match var_type {
        STRING_VAR_TYPE => {
            let var_value_string = get_solo_item(var_value, Rule::var_value_string)?;
            Ok(VarValue::String(
                get_solo_item(var_value_string, Rule::var_value_string_val)?
                    .as_str()
                    .into(),
            ))
        }
        I64_VAR_TYPE => {
            let var_value_i64 = get_solo_item(var_value, Rule::var_value_i64)?.as_str();

            let num: i64 = var_value_i64.parse().map_err(|e: ParseIntError| {
                PetraFrontendError::ProcessError(format!("failed processing number: {e}"))
            })?;
            Ok(VarValue::Integer64(num))
        }
        _ => Err(PetraFrontendError::ProcessError(format!(
            "invalid var type: {var_type}"
        ))),
    }
}

#[derive(Error, Debug)]
pub enum PetraFrontendError {
    #[error("failed reading input")]
    InputError(#[source] std::io::Error),
    #[error("failed parsing input using pest parser")]
    ParseError(#[from] Box<pest::error::Error<Rule>>),
    #[error("failed processing parsed input: {0}")]
    ProcessError(String),
    #[error("unknown petra frontend error")]
    Unknown,
}

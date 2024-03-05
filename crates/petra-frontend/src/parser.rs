use pest::Parser;
use pest_derive::Parser;

use petra_core::{
    Document, EnumDefine, Name, TopItem, ValueOrPointer, ValueRef, VarDeclaration, VarValue,
};
use std::io::Read;
use std::num::ParseIntError;
use thiserror::Error;
use tracing::error;

#[derive(Parser)]
#[grammar = "petra.pest"]
struct PetraParser;

pub fn parse<R: Read>(mut reader: R) -> Result<Document> {
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
                get_solo_item(pair, Some(Rule::new_line_comment_val))?
                    .as_str()
                    .to_string(),
            )),
            Rule::multi_line_comment => Some(TopItem::MultiLineComment(
                get_solo_item(pair, Some(Rule::multi_line_comment_val))?
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
fn parse_var_declaration(mut pairs: pest::iterators::Pairs<Rule>) -> Result<VarDeclaration> {
    let name = verify_type(&mut pairs, Rule::var_name)?
        .as_str()
        .to_string();
    Ok(VarDeclaration::new(
        name.into(),
        parse_var_value_type(pairs)?,
    ))
}
fn parse_var_value_type(mut pairs: pest::iterators::Pairs<Rule>) -> Result<VarValue> {
    let type_and_value = next_item(&mut pairs)?;
    force_empty(&mut pairs)?;

    match type_and_value.as_rule() {
        Rule::tv_i64 => handle_tv_i64(type_and_value),
        Rule::tv_string => handle_tv_string(type_and_value),
        Rule::tv_enum_string => handle_tv_enum_string(type_and_value),
        Rule::tv_enum_i64 => handle_tv_enum_i64(type_and_value),
        Rule::tv_list_string => handle_tv_list_string(type_and_value),
        Rule::tv_list_i64 => handle_tv_list_i64(type_and_value),
        Rule::tv_enum_value => handle_tv_enum_value(type_and_value),
        _ => PetraFrontendError::unexpected_rule(
            type_and_value.as_rule(),
            vec![
                Rule::tv_i64,
                Rule::tv_string,
                Rule::tv_enum_string,
                Rule::tv_enum_i64,
                Rule::tv_list_string,
                Rule::tv_list_i64,
                Rule::tv_enum_value,
            ],
        ),
    }
}
fn verify_type_single<'a>(
    pair: &'a pest::iterators::Pair<'a, Rule>,
    rule_type: Rule,
) -> Result<()> {
    if pair.as_rule() == rule_type {
        Ok(())
    } else {
        PetraFrontendError::unexpected_rule(pair.as_rule(), vec![rule_type])
    }
}

fn verify_type<'a>(
    pairs: &mut pest::iterators::Pairs<'a, Rule>,
    rule_type: Rule,
) -> Result<pest::iterators::Pair<'a, Rule>> {
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
    rule_type: Option<Rule>,
) -> Result<pest::iterators::Pair<'_, Rule>> {
    let mut pairs = pair.into_inner();
    let res = if let Some(rule_type) = rule_type {
        verify_type(&mut pairs, rule_type)?
    } else {
        next_item(&mut pairs)?
    };
    force_empty(&mut pairs)?;
    Ok(res)
}
fn next_item<T>(mut iter: impl Iterator<Item = T>) -> Result<T> {
    iter.next()
        .ok_or_else(|| PetraFrontendError::ProcessError("Not enough items to process".into()))
}
fn force_empty<'a>(mut iter: &mut pest::iterators::Pairs<'a, Rule>) -> Result<()> {
    match iter.next() {
        Some(pair) => Err(PetraFrontendError::ProcessError(format!(
            "too much items to process, got {:?}",
            pair.as_rule()
        ))),
        None => Ok(()),
    }
}
fn parse_enum(
    pair: pest::iterators::Pair<'_, Rule>,
) -> Result<(Option<Name>, pest::iterators::Pair<'_, Rule>)> {
    let mut pairs = pair.into_inner();
    let pair = next_item(&mut pairs)?;
    let res = if pair.as_rule() == Rule::enum_extends {
        let pair = get_solo_item(pair, Some(Rule::var_name))?
            .as_str()
            .to_string();
        (Some(pair.into()), next_item(&mut pairs)?)
    } else {
        (None, pair)
    };
    force_empty(&mut pairs)?;
    Ok(res)
}
fn handle_tv_enum_value(pair: pest::iterators::Pair<'_, Rule>) -> Result<VarValue> {
    let mut pairs = pair.into_inner();
    let pair = verify_type(&mut pairs, Rule::enum_value_type)?;
    let enum_type: Name = get_solo_item(pair, Some(Rule::var_name))?
        .as_str()
        .to_string()
        .into();
    let pair = verify_type(&mut pairs, Rule::ref_value_enum)?;
    force_empty(&mut pairs)?;
    let variant = handle_ref_value_enum(pair)?;
    if enum_type == variant.0 {
        Ok(VarValue::EnumValue(variant))
    } else {
        PetraFrontendError::wrong_enum_value_type(enum_type, variant.0)
    }
}

fn handle_list<T>(pair: pest::iterators::Pair<'_, Rule>) -> Result<Vec<ValueOrPointer<T>>>
where
    T: ValueParser<T>,
{
    let pair = get_solo_item(pair, Some(T::list_rule_type()))?;
    let mut values: Vec<ValueOrPointer<T>> = vec![];
    for pair in pair.into_inner() {
        verify_type_single(&pair, T::list_item_rule_type())?;
        let pair = get_solo_item(pair, None)?;
        let ref_or_value = handle_value_and_ref(pair)?;
        values.push(ref_or_value);
    }
    Ok(values)
}
fn handle_tv_list_string(pair: pest::iterators::Pair<'_, Rule>) -> Result<VarValue> {
    Ok(VarValue::ListString(handle_list(pair)?))
}
fn handle_tv_list_i64(pair: pest::iterators::Pair<'_, Rule>) -> Result<VarValue> {
    Ok(VarValue::ListInteger64(handle_list(pair)?))
}

fn handle_enum<T>(pair: pest::iterators::Pair<'_, Rule>) -> Result<EnumDefine<T>>
where
    T: ValueParser<T>,
{
    let (extends, pair) = parse_enum(pair)?;
    verify_type_single(&pair, T::enum_rule_type())?;
    let mut variants: Vec<(petra_core::Name, ValueOrPointer<T>)> = vec![];
    for pair in pair.into_inner() {
        verify_type_single(&pair, T::enum_item_rule_type())?;
        let mut pairs = pair.into_inner();
        let variant_name = verify_type(&mut pairs, Rule::var_name)?
            .as_str()
            .to_string();
        let pair = next_item(&mut pairs)?;
        let ref_or_value = handle_value_and_ref(pair)?;
        force_empty(&mut pairs)?;
        variants.push((variant_name.into(), ref_or_value));
    }
    Ok(EnumDefine { variants, extends })
}
fn handle_tv_enum_string(pair: pest::iterators::Pair<'_, Rule>) -> Result<VarValue> {
    Ok(VarValue::EnumString(handle_enum(pair)?))
}
fn handle_tv_enum_i64(pair: pest::iterators::Pair<'_, Rule>) -> Result<VarValue> {
    Ok(VarValue::EnumInteger64(handle_enum(pair)?))
}

fn handle_value_and_ref<T>(pair: pest::iterators::Pair<'_, Rule>) -> Result<ValueOrPointer<T>>
where
    T: ValueParser<T>,
{
    if pair.as_rule() == T::primitive_rule_type() {
        Ok(ValueOrPointer::Raw(T::parse(pair)?))
    } else {
        match pair.as_rule() {
            Rule::ref_value_var => Ok(handle_ref_value_var(pair)?),
            Rule::ref_value_enum => Ok(ValueOrPointer::Ref(ValueRef::EnumVariant(
                handle_ref_value_enum(pair)?,
            ))),
            _ => PetraFrontendError::unexpected_rule(
                pair.as_rule(),
                vec![
                    T::primitive_rule_type(),
                    Rule::ref_value_var,
                    Rule::ref_value_enum,
                ],
            ),
        }
    }
}
fn handle_tv_i64(pair: pest::iterators::Pair<'_, Rule>) -> Result<VarValue> {
    let pair = get_solo_item(pair, None)?;
    Ok(VarValue::Integer64(handle_value_and_ref(pair)?))
}
fn handle_tv_string(pair: pest::iterators::Pair<'_, Rule>) -> Result<VarValue> {
    let pair = get_solo_item(pair, None)?;
    Ok(VarValue::String(handle_value_and_ref(pair)?))
}

fn handle_ref_value_var<T>(pair: pest::iterators::Pair<'_, Rule>) -> Result<ValueOrPointer<T>> {
    let name: String = get_solo_item(pair, Some(Rule::var_name))?
        .as_str()
        .to_string();
    Ok(ValueOrPointer::Ref(ValueRef::Primitive(name.into())))
}
fn handle_ref_value_enum(pair: pest::iterators::Pair<'_, Rule>) -> Result<(Name, Name)> {
    let mut pairs = pair.into_inner();
    let enum_name = verify_type(&mut pairs, Rule::var_name)?
        .as_str()
        .to_string();
    let variant_name = verify_type(&mut pairs, Rule::var_name)?
        .as_str()
        .to_string();
    force_empty(&mut pairs)?;
    Ok((enum_name.into(), variant_name.into()))
}

type Result<T> = std::result::Result<T, PetraFrontendError>;
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
impl PetraFrontendError {
    fn wrong_enum_value_type<T>(declaration: Name, value: Name) -> Result<T> {
        Err(PetraFrontendError::ProcessError(format!(
            "enum value got different types for declaration: declaration={declaration:?}, value={value:?}"
        )))
    }

    fn unexpected_rule<T>(actual: Rule, expected: Vec<Rule>) -> Result<T> {
        if expected.len() == 1 {
            Err(PetraFrontendError::ProcessError(format!(
                "got wrong rule type, expected={:?}, actual={actual:?}",
                expected.first().unwrap()
            )))
        } else {
            Err(PetraFrontendError::ProcessError(format!(
                "got wrong rule type for expected={expected:?}, actual={actual:?}",
            )))
        }
    }
}
trait ValueParser<T> {
    fn primitive_rule_type() -> Rule;
    fn enum_rule_type() -> Rule;
    fn enum_item_rule_type() -> Rule;
    fn list_rule_type() -> Rule;
    fn list_item_rule_type() -> Rule;

    fn parse(pair: pest::iterators::Pair<'_, Rule>) -> Result<T>;
}
impl ValueParser<i64> for i64 {
    fn primitive_rule_type() -> Rule {
        Rule::val_i64
    }
    fn enum_rule_type() -> Rule {
        Rule::val_enum_i64
    }
    fn enum_item_rule_type() -> Rule {
        Rule::val_enum_i64_item
    }
    fn list_rule_type() -> Rule {
        Rule::val_list_i64
    }
    fn list_item_rule_type() -> Rule {
        Rule::val_list_i64_item
    }

    fn parse(pair: pest::iterators::Pair<'_, Rule>) -> Result<i64> {
        let num: i64 = pair.as_str().parse().map_err(|e: ParseIntError| {
            PetraFrontendError::ProcessError(format!("failed processing number: {e}"))
        })?;
        Ok(num)
    }
}

impl ValueParser<String> for String {
    fn primitive_rule_type() -> Rule {
        Rule::val_string
    }
    fn enum_rule_type() -> Rule {
        Rule::val_enum_string
    }
    fn enum_item_rule_type() -> Rule {
        Rule::val_enum_string_item
    }
    fn list_rule_type() -> Rule {
        Rule::val_list_string
    }
    fn list_item_rule_type() -> Rule {
        Rule::val_list_string_item
    }
    fn parse(pair: pest::iterators::Pair<'_, Rule>) -> Result<String> {
        let var_value_string = get_solo_item(pair, Some(Rule::val_string_inner))?.as_str();
        Ok(var_value_string.into())
    }
}

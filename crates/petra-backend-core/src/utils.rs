use convert_case::{Case, Casing};
#[must_use]
pub fn to_upper_snake(val: &str) -> String {
    val.to_case(Case::UpperSnake)
}

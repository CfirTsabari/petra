use convert_case::{Case, Casing};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Document {
    pub items: Vec<TopItem>,
}
impl Document {
    #[must_use]
    pub const fn new() -> Self {
        Self { items: vec![] }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VarDeclaration {
    pub name: Name,
    pub value: VarValue,
}

impl VarDeclaration {
    #[must_use]
    pub const fn new(name: String, value: VarValue) -> Self {
        Self {
            name: Name::new(name),
            value,
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValueRef {
    Primitive(Name),
    EnumVariant((Name, Name)),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VarValue {
    String(ValueOrPointer<String>),
    Integer64(ValueOrPointer<i64>),
    EnumValue((Name, Name)),
    EnumString(EnumDefine<String>),
    EnumInteger64(EnumDefine<i64>),
    ListString(Vec<ValueOrPointer<String>>),
    ListInteger64(Vec<ValueOrPointer<i64>>),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EnumDefine<T> {
    pub extends: Option<Name>,
    pub variants: Vec<(Name, ValueOrPointer<T>)>,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValueOrPointer<T> {
    Ref(ValueRef),
    Raw(T),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TopItem {
    Comment(String),
    MultiLineComment(String),
    VarDeclaration(VarDeclaration),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Name {
    data: String,
}
impl Name {
    #[must_use]
    pub const fn new(input: String) -> Self {
        Self { data: input }
    }
    #[must_use]
    pub fn to_upper_snake(&self) -> String {
        self.data.as_str().to_case(Case::UpperSnake)
    }
    #[must_use]
    pub fn to_pascal_case(&self) -> String {
        self.data.as_str().to_case(Case::Pascal)
    }
    #[must_use]
    pub fn to_lower_snake(&self) -> String {
        self.data.as_str().to_case(Case::Snake)
    }
}

impl From<String> for Name {
    fn from(val: String) -> Self {
        Self::new(val)
    }
}

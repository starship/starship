use std::borrow::Cow;
use std::collections::BTreeSet;

/// Type that holds a number of variables of type `T`
pub trait VariableHolder<T> {
    fn get_variables(&self) -> BTreeSet<T>;
}

/// Type that holds a number of style variables of type `T`
pub trait StyleVariableHolder<T> {
    fn get_style_variables(&self) -> BTreeSet<T>;
}

#[derive(Clone)]
pub struct TextGroup<'a> {
    pub format: Vec<FormatElement<'a>>,
    pub style: Vec<StyleElement<'a>>,
}

#[derive(Clone)]
pub enum FormatElement<'a> {
    Text(Cow<'a, str>),
    Variable(Cow<'a, str>),
    TextGroup(TextGroup<'a>),
    Conditional(Vec<FormatElement<'a>>),
}

#[derive(Clone)]
pub enum StyleElement<'a> {
    Text(Cow<'a, str>),
    Variable(Cow<'a, str>),
}

impl<'a> VariableHolder<Cow<'a, str>> for FormatElement<'a> {
    fn get_variables(&self) -> BTreeSet<Cow<'a, str>> {
        match self {
            FormatElement::Variable(var) => {
                let mut variables = BTreeSet::new();
                variables.insert(var.clone());
                variables
            }
            FormatElement::TextGroup(textgroup) => textgroup.format.get_variables(),
            FormatElement::Conditional(format) => format.get_variables(),
            _ => Default::default(),
        }
    }
}

impl<'a> VariableHolder<Cow<'a, str>> for Vec<FormatElement<'a>> {
    fn get_variables(&self) -> BTreeSet<Cow<'a, str>> {
        self.iter().fold(BTreeSet::new(), |mut acc, el| {
            acc.extend(el.get_variables());
            acc
        })
    }
}

impl<'a> VariableHolder<Cow<'a, str>> for &[FormatElement<'a>] {
    fn get_variables(&self) -> BTreeSet<Cow<'a, str>> {
        self.iter().fold(BTreeSet::new(), |mut acc, el| {
            acc.extend(el.get_variables());
            acc
        })
    }
}

impl<'a> StyleVariableHolder<Cow<'a, str>> for StyleElement<'a> {
    fn get_style_variables(&self) -> BTreeSet<Cow<'a, str>> {
        match self {
            StyleElement::Variable(var) => {
                let mut variables = BTreeSet::new();
                variables.insert(var.clone());
                variables
            }
            _ => Default::default(),
        }
    }
}

impl<'a> StyleVariableHolder<Cow<'a, str>> for Vec<StyleElement<'a>> {
    fn get_style_variables(&self) -> BTreeSet<Cow<'a, str>> {
        self.iter().fold(BTreeSet::new(), |mut acc, el| {
            acc.extend(el.get_style_variables());
            acc
        })
    }
}

impl<'a> StyleVariableHolder<Cow<'a, str>> for Vec<FormatElement<'a>> {
    fn get_style_variables(&self) -> BTreeSet<Cow<'a, str>> {
        self.iter().fold(BTreeSet::new(), |mut acc, el| match el {
            FormatElement::TextGroup(textgroup) => {
                acc.extend(textgroup.style.get_style_variables());
                acc.extend(textgroup.format.get_style_variables());
                acc
            }
            FormatElement::Conditional(format) => {
                acc.extend(format.get_style_variables());
                acc
            }
            _ => acc,
        })
    }
}

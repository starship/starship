use std::borrow::Cow;

pub struct TextGroup<'a> {
    pub format: Vec<FormatElement<'a>>,
    pub style: Vec<StyleElement<'a>>,
}

pub enum FormatElement<'a> {
    Text(Cow<'a, str>),
    Variable(Cow<'a, str>),
    TextGroup(TextGroup<'a>),
}

pub enum StyleElement<'a> {
    Text(Cow<'a, str>),
    Variable(Cow<'a, str>),
}

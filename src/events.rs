#[derive(Debug, PartialEq)]
pub enum ElementEvent<'a> {
    Start(ElementStart),

    // Ends the most recently started element.
    End,

    Text(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct ElementStart {
    variant: ElementStartVariant,
}

impl ElementStart {
    pub fn simple(variant: ElementStartVariant) -> ElementStart {
        ElementStart { variant }
    }
}

#[derive(Debug, PartialEq)]
pub enum ElementStartVariant {
    Paragraph,
}

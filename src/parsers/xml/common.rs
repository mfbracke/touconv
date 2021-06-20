/// Shared state between the parser and the different subparsers.
pub struct ParseState {
    /// The open tags in the order in which they were opened.
    open_tags: Vec<Box<Tag>>,
}

impl ParseState {
    pub fn new() -> ParseState {
        ParseState {
            open_tags: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn with_open_tags(tags: Vec<Tag>) -> ParseState {
        ParseState {
            open_tags: tags.into_iter().map(|tag| Box::new(tag)).collect(),
        }
    }

    pub fn last_opened_tag(&self) -> Option<&Tag> {
        self.open_tags.last().map(|tag| tag.as_ref())
    }

    pub fn open_tag(&mut self, tag: Box<Tag>) {
        self.open_tags.push(tag);
    }

    pub fn close_tag(&mut self) -> Option<Box<Tag>> {
        self.open_tags.pop()
    }
}

/// Events for the subparsers
pub enum Event<'a> {
    TagAboutToOpen(&'a Tag),
    TagClosed(&'a Tag),
    Text(&'a str),
}

pub struct Tag {
    namespace: String,
    name: String,
}

impl Tag {
    pub fn new(namespace: String, name: String) -> Tag {
        Tag { namespace, name }
    }

    #[cfg(test)]
    pub fn from_refs(namespace: &str, name: &str) -> Tag {
        Tag {
            namespace: namespace.to_string(),
            name: name.to_string(),
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn matches(&self, namespace: &str, name: &str) -> bool {
        namespace == self.namespace() && name == self.name
    }
}

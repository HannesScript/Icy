#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextNode {
    pub text: String,
}

impl TextNode {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementType {
    Root,
    H1,
    Paragraph,
    Button,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentNode {
    pub element: ElementType,
    pub text: Option<TextNode>,
    pub children: Vec<ComponentNode>,
}

impl ComponentNode {
    pub fn root(children: Vec<ComponentNode>) -> Self {
        Self {
            element: ElementType::Root,
            text: None,
            children,
        }
    }

    pub fn h1(text: impl Into<String>) -> Self {
        Self {
            element: ElementType::H1,
            text: Some(TextNode::new(text)),
            children: vec![],
        }
    }

    pub fn paragraph(text: impl Into<String>) -> Self {
        Self {
            element: ElementType::Paragraph,
            text: Some(TextNode::new(text)),
            children: vec![],
        }
    }

    pub fn button(text: impl Into<String>) -> Self {
        Self {
            element: ElementType::Button,
            text: Some(TextNode::new(text)),
            children: vec![],
        }
    }

    pub fn with_children(mut self, children: Vec<ComponentNode>) -> Self {
        self.children = children;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.text.is_none() && self.children.is_empty() && self.element != ElementType::Root {
            return Err("non-root component must contain text or children".to_string());
        }

        for child in &self.children {
            child.validate()?;
        }

        Ok(())
    }
}

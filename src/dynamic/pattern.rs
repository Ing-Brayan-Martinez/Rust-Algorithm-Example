
// root element
pub enum Element {
    Html(HtmlElement),
    Internal(InternalElement),
    Custom(CustomElement),
}

trait Render {
    fn render(&self) -> String;
}

// html element
pub enum HtmlElement {
    Div(Div),
    Span(Span),
    Paragraph(Paragraph),
    Image(Image),
}

impl Render for HtmlElement {
    fn render(&self) -> String {
        match self {
            HtmlElement::Div(div) => div.render(),
            HtmlElement::Span(span) => span.render(),
            HtmlElement::Paragraph(paragraph) => paragraph.render(),
            HtmlElement::Image(image) => image.render(),
        }
    }
}

// Concrete HtmlElements
pub struct Div {}

impl Render for Div {
    fn render(&self) -> String {
        String::from("")
    }
}

pub struct Span {}

impl Render for Span {
    fn render(&self) -> String {
        String::from("")
    }
}

pub struct Paragraph {}

impl Render for Paragraph {
    fn render(&self) -> String {
        String::from("")
    }
}

pub struct Image {}

impl Render for Image {
    fn render(&self) -> String {
        String::from("")
    }
}

// internal element
pub enum InternalElement {
    Text(Text),
    HtmlLiteral(HtmlLiteral),
    Nothing(Nothing),
}

impl Render for InternalElement {
    fn render(&self) -> String {
        match self {
            InternalElement::Text(text) => text.render(),
            InternalElement::HtmlLiteral(html_literal) => html_literal.render(),
            InternalElement::Nothing(nothing) => nothing.render(),
        }
    }
}


// Concrete InternalElements
pub struct Text {}

impl Render for Text {
    fn render(&self) -> String {
        String::from("")
    }
}

pub struct HtmlLiteral {}

impl Render for HtmlLiteral {
    fn render(&self) -> String {
        String::from("")
    }
}

pub struct Nothing {}

impl Render for Nothing {
    fn render(&self) -> String {
        String::from("")
    }
}


// custom element
pub enum CustomElement {
    // Currently no variants, mirroring the Dart code.
    // If you need to represent specific custom elements, you can add variants here.
}

impl Render for CustomElement {
    fn render(&self) -> String {
        // In Dart code, it's custom.toString().  Since CustomElement is an enum,
        // and we don't have specific data in variants, we can just return a placeholder.
        String::from("<custom-element>") // Or decide how CustomElement should be represented as String
    }
}


pub fn get_element_string(element: Element) -> String {
    match element {
        Element::Html(html) => match html {
            HtmlElement::Div(div) => format!("<div>{}</div>", div.render()),
            HtmlElement::Span(span) => format!("<span>{}</span>", span.render()),
            HtmlElement::Paragraph(paragraph) => format!("<p>{}</p>", paragraph.render()),
            HtmlElement::Image(image) => format!("<img src=\"{}\" />", image.render()),
        },
        Element::Internal(internal) => match internal {
            InternalElement::Text(text) => text.render(),
            InternalElement::HtmlLiteral(html_literal) => html_literal.render(),
            InternalElement::Nothing(nothing) => nothing.render(),
        },
        Element::Custom(custom) => custom.render(),
    }
}


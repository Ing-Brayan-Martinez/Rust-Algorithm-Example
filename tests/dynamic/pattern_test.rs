use rust_algorithm_example::dynamic::pattern::get_element_string;
use rust_algorithm_example::dynamic::pattern::Div;
use rust_algorithm_example::dynamic::pattern::Element;
use rust_algorithm_example::dynamic::pattern::HtmlElement;
use rust_algorithm_example::dynamic::pattern::HtmlLiteral;
use rust_algorithm_example::dynamic::pattern::Image;
use rust_algorithm_example::dynamic::pattern::InternalElement;
use rust_algorithm_example::dynamic::pattern::Nothing;
use rust_algorithm_example::dynamic::pattern::Paragraph;
use rust_algorithm_example::dynamic::pattern::Span;
use rust_algorithm_example::dynamic::pattern::Text;

#[test]
fn pattern_test() {
    let div_element = Element::Html(HtmlElement::Div(Div {}));
    let span_element = Element::Html(HtmlElement::Span(Span {}));
    let paragraph_element = Element::Html(HtmlElement::Paragraph(Paragraph {}));
    let image_element = Element::Html(HtmlElement::Image(Image {}));
    let text_element = Element::Internal(InternalElement::Text(Text {}));
    let html_literal_element = Element::Internal(InternalElement::HtmlLiteral(HtmlLiteral {}));
    let nothing_element = Element::Internal(InternalElement::Nothing(Nothing {}));
    //let custom_element = Element::Custom(CustomElement {});

    println!("Div Element: {}", get_element_string(div_element));
    println!("Span Element: {}", get_element_string(span_element));
    println!("Paragraph Element: {}", get_element_string(paragraph_element));
    println!("Image Element: {}", get_element_string(image_element));
    println!("Text Element: {}", get_element_string(text_element));
    println!("HtmlLiteral Element: {}", get_element_string(html_literal_element));
    println!("Nothing Element: {}", get_element_string(nothing_element));
    //println!("Custom Element: {}", get_element_string(custom_element));
}
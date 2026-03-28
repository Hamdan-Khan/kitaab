use crate::Message;
use crate::parser::MdElement;
use iced::{
    Color, Element, Length, Theme, Font,
    widget::{column, container, text},
};

pub fn render_md(elements: &[MdElement]) -> Element<'_, Message> {
    let mut col = column![].spacing(12).width(Length::Fill);

    for el in elements {
        col = col.push(render_element(el));
    }

    container(col).padding(16).width(Length::Fill).into()
}

fn render_element(el: &MdElement) -> Element<'_, Message> {
    match el {
        MdElement::Paragraph(content) => text(content).size(16).width(Length::Fill).into(),

        MdElement::Heading { level, content } => {
            let size = match level {
                1 => 32,
                2 => 28,
                3 => 24,
                4 => 20,
                _ => 18,
            };

            text(content).size(size).width(Length::Fill).into()
        }

        MdElement::CodeBlock(code) => container(text(code).size(14).font(Font::MONOSPACE))
            .padding(12)
            .width(Length::Fill)
            .style(|_theme: &Theme| container::Appearance {
                background: Some(Color::from_rgb(0.1, 0.1, 0.1).into()),
                text_color: Some(Color::from_rgb(0.9, 0.9, 0.9).into()),
                ..Default::default()
            })
            .into(),
    }
}

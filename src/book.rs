use crate::{
    page_loader::PageLoader,
    parser::{self, ParsedContent},
};

pub struct Book<L: PageLoader> {
    loader: L,
    current: u32,
}

impl<L: PageLoader> Book<L> {
    pub fn new(loader: L) -> Self {
        Self { loader, current: 0 }
    }

    pub fn next_page(&mut self) -> Option<ParsedContent> {
        self.current += 1;
        self.load_page()
    }

    pub fn prev_page(&mut self) -> Option<ParsedContent> {
        if self.current > 0 {
            self.current -= 1;
        }
        self.load_page()
    }

    pub fn load_page(&self) -> Option<ParsedContent> {
        let page = self.loader.load_page(self.current)?;
        let parsed = parser::ParsedContent::parse_md(page);
        Some(parsed)
    }

    pub fn get_current(&self) -> u32 {
        self.current
    }
}

use std::ops::Deref;

use scraper::{error::SelectorErrorKind, Selector};

pub struct WgetSelector(Selector);

impl WgetSelector {
    pub fn new(selectors: &str) -> Result<WgetSelector, SelectorErrorKind> {
        let selector = Selector::parse(selectors)?;
        Ok(WgetSelector(selector))
    }
}

impl Deref for WgetSelector {
    type Target = Selector;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

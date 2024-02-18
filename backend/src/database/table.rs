use super::page::Page;

#[derive(Debug, Clone)]
pub(super) struct Table {
    pub pages: Vec<Page>,
}

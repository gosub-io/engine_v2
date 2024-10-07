#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
    EqualsTag(String),
    EqualsId(String),
    ContainsClass(String),
    ContainsAttribute(String),
    ContainsChildTag(String),
    HasParentTag(String),
}

impl gosub_shared::traits::document::query::Condition for Condition {
    fn equals_tag(tag_name: &str) -> Self {
        Condition::EqualsTag(tag_name.to_owned())
    }

    fn equals_id(id: &str) -> Self {
        Condition::EqualsId(id.to_owned())
    }

    fn contains_class(class: &str) -> Self {
        Condition::ContainsClass(class.to_owned())
    }

    fn contains_attribute(attribute: &str) -> Self {
        Condition::ContainsAttribute(attribute.to_owned())
    }

    fn contains_child_tag(child_tag: &str) -> Self {
        Condition::ContainsChildTag(child_tag.to_owned())
    }

    fn has_parent_tag(parent_tag: &str) -> Self {
        Condition::HasParentTag(parent_tag.to_owned())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SearchType {
    Uninitialized,
    FindFirst,
    FindAll,
}

impl gosub_shared::traits::document::query::SearchType for SearchType {
    fn uninitialized() -> Self {
        SearchType::Uninitialized
    }

    fn find_first() -> Self {
        SearchType::FindFirst
    }

    fn find_all() -> Self {
        SearchType::FindAll
    }
}

pub struct Query {
    pub conditions: Vec<Condition>,
    pub search_type: SearchType,
}

impl gosub_shared::traits::document::query::Query for Query {
    type SearchType = SearchType;
    type Condition = Condition;

    fn new(search_type: Self::SearchType, conditions: Vec<Self::Condition>) -> Self {
        Self {
            search_type,
            conditions,
        }
    }

    fn search_type(&self) -> SearchType {
        self.search_type.clone()
    }

    fn conditions(&self) -> Vec<Condition> {
        self.conditions.clone()
    }
}


#[allow(dead_code)]
impl Query {
    pub(crate) fn new() -> Self {
        Self {
            conditions: Vec::new(),
            search_type: SearchType::Uninitialized,
        }
    }

    pub(crate) fn equals_tag(mut self, tag_name: &str) -> Self {
        self.conditions.push(Condition::EqualsTag(tag_name.to_owned()));
        self
    }

    pub(crate) fn equals_id(mut self, id: &str) -> Self {
        self.conditions.push(Condition::EqualsId(id.to_owned()));
        self
    }

    pub(crate) fn contains_class(mut self, class: &str) -> Self {
        self.conditions.push(Condition::ContainsClass(class.to_owned()));
        self
    }

    pub(crate) fn contains_attribute(mut self, attribute: &str) -> Self {
        self.conditions.push(Condition::ContainsAttribute(attribute.to_owned()));
        self
    }

    pub(crate) fn contains_child_tag(mut self, child_tag: &str) -> Self {
        self.conditions.push(Condition::ContainsChildTag(child_tag.to_owned()));
        self
    }

    pub(crate) fn has_parent_tag(mut self, parent_tag: &str) -> Self {
        self.conditions.push(Condition::HasParentTag(parent_tag.to_owned()));
        self
    }

    pub(crate) fn find_first(mut self) -> Self {
        self.search_type = SearchType::FindFirst;
        self
    }

    pub(crate) fn find_all(mut self) -> Self {
        self.search_type = SearchType::FindAll;
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uninitialized() {
        let query = Query::new().equals_tag("div").equals_id("myid");
        assert_eq!(query.search_type, SearchType::Uninitialized);
    }

    #[test]
    fn find_first() {
        let query = Query::new().find_first();
        assert_eq!(query.search_type, SearchType::FindFirst);
    }

    #[test]
    fn find_all() {
        let query = Query::new().find_all();
        assert_eq!(query.search_type, SearchType::FindAll);
    }

    #[test]
    fn build_conditions() {
        let query = Query::new()
            .equals_tag("div")
            .equals_id("myid")
            .contains_class("myclass")
            .contains_attribute("myattr")
            .contains_child_tag("h1")
            .has_parent_tag("html")
            .find_first();

        assert_eq!(query.conditions.len(), 6);
        assert_eq!(query.conditions[0], Condition::EqualsTag("div".to_owned()));
        assert_eq!(query.conditions[1], Condition::EqualsId("myid".to_owned()));
        assert_eq!(query.conditions[2], Condition::ContainsClass("myclass".to_owned()));
        assert_eq!(query.conditions[3], Condition::ContainsAttribute("myattr".to_owned()));
        assert_eq!(query.conditions[4], Condition::ContainsChildTag("h1".to_owned()));
        assert_eq!(query.conditions[5], Condition::HasParentTag("html".to_owned()));
    }
}

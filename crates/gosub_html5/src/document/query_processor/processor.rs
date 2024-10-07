use anyhow::Error;
use gosub_shared::document::DocumentHandle;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::document::query::{Query, SearchType};
use gosub_shared::traits::document::HasDocument;
use crate::document::tree_iterator::TreeIterator;

pub struct QueryProcessor<C: HasDocument, Q: Query> {
    handle: DocumentHandle<C>,
    _marker: std::marker::PhantomData<Q>,
}

impl <C: HasDocument, Q: Query>  QueryProcessor<C, Q> {
    pub fn new(handle: DocumentHandle<C>) -> Self {
        Self {
            handle,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn query(&self, query: &Q) -> Result<Vec<NodeId>, Error> {
        if query.search_type() == Q::SearchType::uninitialized() {
            return Err(anyhow::anyhow!("Query predicate is uninitialized").into());
        }

        let tree_iterator = TreeIterator::new(self.handle.clone());

        let mut found_ids = Vec::new();
        for current_node_id in tree_iterator {
            let mut predicate_result: bool = true;
            for condition in &query.conditions() {
                if !self.matches_query_condition(&current_node_id, condition) {
                    predicate_result = false;
                    break;
                }
            }

            if predicate_result {
                found_ids.push(current_node_id);
                if query.search_type() == SearchType::find_first() {
                    return Ok(found_ids);
                }
            }
        }

        Ok(found_ids)
    }

    fn matches_query_condition(&self, _node_id: &NodeId, _condition: &Q::Condition) -> bool {
        todo!()
    }
}



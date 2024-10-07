use anyhow::Error;
use gosub_shared::document::DocumentHandle;
use gosub_shared::node_id::NodeId;
use gosub_shared::traits::document::HasDocument;
use crate::document::query_processor::query::{Condition, SearchType};
use crate::document::tree_iterator::TreeIterator;

pub struct QueryProcessor<C: HasDocument> {
    _phantom: std::marker::PhantomData<C>
}

impl <C: HasDocument> gosub_shared::traits::document::QueryProcessor<C> for QueryProcessor<C> {
    fn query(handle: DocumentHandle<C>, query: &impl gosub_shared::traits::document::Query) -> Result<Vec<NodeId>, Error> {
        if query.search_type() == SearchType::Uninitialized {
            return Err(anyhow::anyhow!("Query predicate is uninitialized").into());
        }

        let tree_iterator = TreeIterator::new(handle.clone());

        let mut found_ids = Vec::new();
        for current_node_id in tree_iterator {
            let mut predicate_result: bool = true;
            for condition in &query.conditions() {
                if !self::matches_query_condition(handle.clone(), &current_node_id, condition) {
                    predicate_result = false;
                    break;
                }
            }

            if predicate_result {
                found_ids.push(current_node_id);
                if query.search_type() == SearchType::FindFirst {
                    return Ok(found_ids);
                }
            }
        }

        Ok(found_ids)
    }
}

fn matches_query_condition<C: HasDocument>(_handle: DocumentHandle<C>, _node_id: &NodeId, _condition: &Condition) -> bool {
    todo!()
}

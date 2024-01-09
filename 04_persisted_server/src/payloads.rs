use serde::{Deserialize, Serialize};

use crate::todos::TodosFilter;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodosFilterRequest {
    pub filter: TodosFilter,
}

impl Default for TodosFilterRequest {
    fn default() -> Self {
        Self {
            filter: TodosFilter::All,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewTodoRequest {
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TodoCompletedRequest {
    pub completed: bool,
}

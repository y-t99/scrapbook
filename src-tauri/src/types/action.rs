use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

pub enum ActionType {
    BlockAdded = 0,
    BlockRemoved = 1,
    BlockMoved = 2,
    BlockChanged = 3,
}

impl TryFrom<usize> for ActionType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == ActionType::BlockAdded as i32 => Ok(ActionType::BlockAdded),
            x if x == ActionType::BlockRemoved as i32 => Ok(ActionType::BlockRemoved),
            x if x == ActionType::BlockMoved as i32 => Ok(ActionType::BlockMoved),
            x if x == ActionType::BlockChanged as i32 => Ok(ActionType::BlockChanged),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    id: String,
    tool: String,
    data: Document,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    action_type: ActionType,
    block: Block,
}

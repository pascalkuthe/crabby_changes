use indexmap::{IndexMap, IndexSet};
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use crate::state::PullRequest;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Change {
    pub message: String,
    pub group: String,
}

pub type ChangeMeta = IndexSet<PullRequest>;

pub fn serialize<S>(map: &IndexMap<Change, ChangeMeta>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_seq(map.iter().map(|(change, meta)| SerializedChange {
        change: change.clone(),
        prs: meta.clone(),
    }))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<IndexMap<Change, ChangeMeta>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut map = IndexMap::new();
    for change in Vec::<SerializedChange>::deserialize(deserializer)? {
        map.insert(change.change, change.prs);
    }
    Ok(map)
}

#[derive(Serialize, Deserialize)]
pub struct SerializedChange {
    #[serde(flatten)]
    change: Change,
    prs: ChangeMeta,
}

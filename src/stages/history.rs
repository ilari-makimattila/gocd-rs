extern crate serde_json;

use super::{Gocd, Result};


#[derive(Deserialize, Debug)]
struct Pagination {
    stages: Vec<StageHistory>
}

#[derive(Deserialize, Debug)]
pub struct StageHistory {
    pub name: String,
    pub approved_by: String,
    pub pipeline_counter: i64,
    pub pipeline_name: String,
    pub result: String,
    pub approval_type: String,
    pub id: i64,
    pub counter: String,
    pub jobs: Vec<StageHistoryJob>
}

#[derive(Deserialize, Debug)]
pub struct StageHistoryJob {
    pub name: String,
    pub result: String,
    pub state: String,
    pub id: i64,
    pub scheduled_date: u64
}


#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;

    #[test]
    fn it_deserializes_from_example() {
        let pagination: Pagination = serde_json::from_str(
            include_str!("../../tests/data/stage_history.json"))
            .unwrap();
        let stage_history = pagination.stages;
        assert_eq!(stage_history[0].name, "defaultStage");
        assert_eq!(stage_history[0].counter, "1");
        assert_eq!(stage_history[0].jobs[0].name, "defaultJob");
    }
}

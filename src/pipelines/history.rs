use super::{Gocd, Result};


#[derive(Deserialize, Debug)]
struct Pagination {
    pipelines: Vec<PipelineHistory>
}

#[derive(Deserialize, Debug)]
pub struct PipelineHistory {
    pub label: String,
    pub id: i64,
    pub counter: i64,
    pub name: String,
    pub natural_order: i64,
    pub can_run: bool,
    pub comment: Option<String>,
    pub stages: Vec<PipelineHistoryStage>
}


#[derive(Deserialize, Debug)]
pub struct PipelineHistoryStage {
    pub name: String,
    pub result: String,
    pub id: i64,
    pub counter: String,
    pub jobs: Vec<PipelineHistoryStageJob>
}


#[derive(Deserialize, Debug)]
pub struct PipelineHistoryStageJob {
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
            include_str!("../../tests/data/pipeline_history.json"))
            .unwrap();
        let pipeline_history = pagination.pipelines;
        assert_eq!(pipeline_history[0].name, "pipeline1");
        assert_eq!(pipeline_history[0].stages[0].jobs[0].result, "Failed");
    }
}

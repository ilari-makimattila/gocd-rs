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


pub trait StageHistories {
    fn stage_history(&self, pipeline: &str, stage_name: &str) -> Result<Vec<StageHistory>>;
}

impl StageHistories for Gocd {
    fn stage_history(&self, pipeline: &str, stage_name: &str) -> Result<Vec<StageHistory>> {
        let data = self.get(format!("go/api/stages/{}/{}/history", pipeline, stage_name).as_str(),
                            Some("application/json"));
        match data {
            Ok(data) => {
                let pagination: Pagination = serde_json::from_str(data.as_str()).unwrap();
                Ok(pagination.stages)
            },
            Err(e) => Err(e)
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate serde_json;

    use mockito::mock;
    use super::*;
    use super::super::super::*;

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

    #[test]
    fn it_fetches_stage_history() {
        let _m = mock("GET", "/go/api/stages/pipeline1/stage1/history")
            .with_status(200)
            .with_body(include_str!("../../tests/data/stage_history.json"))
            .create();

        let gocd = Gocd::new(mockito::SERVER_URL, "foo", "bar");
        let stage_history = gocd.stage_history("pipeline1", "stage1").unwrap();

        assert_eq!(stage_history[0].name, "defaultStage");
        assert_eq!(stage_history[0].pipeline_name, "mypipeline");
    }
}

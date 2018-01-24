extern crate serde_json;

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


pub trait PipelineHistories {
    fn pipeline_history(&self, pipeline: &str) -> Result<Vec<PipelineHistory>>;
}

impl PipelineHistories for Gocd {
    fn pipeline_history(&self, pipeline: &str) -> Result<Vec<PipelineHistory>> {
        let data = self.get(format!("go/api/pipelines/{}/history", pipeline).as_str(),
                            Some("application/json"));
        match data {
            Ok(data) => {
                let pagination: Pagination = serde_json::from_str(data.as_str()).unwrap();
                Ok(pagination.pipelines)
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
            include_str!("../../tests/data/pipeline_history.json"))
            .unwrap();
        let pipeline_history = pagination.pipelines;
        assert_eq!(pipeline_history[0].name, "pipeline1");
        assert_eq!(pipeline_history[0].stages[0].jobs[0].result, "Failed");
    }

    #[test]
    fn it_fetches_pipeline_history() {
        let _m = mock("GET", "/go/api/pipelines/pipeline1/history")
            .with_status(200)
            .with_body(include_str!("../../tests/data/pipeline_history.json"))
            .create();

        let gocd = Gocd::new(mockito::SERVER_URL, "foo", "bar");
        let pipeline_history = gocd.pipeline_history("pipeline1").unwrap();

        assert_eq!(pipeline_history[0].name, "pipeline1");
        assert_eq!(pipeline_history[0].stages[0].name, "stage1");
    }
}

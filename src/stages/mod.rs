extern crate serde;
extern crate serde_json;

use super::{Gocd, Result};


pub mod history;


#[derive(Deserialize, Debug)]
pub struct Stage {
    pub name: String,
    pub approved_by: String,
    pub pipeline_counter: i64,
    pub pipeline_name: String,
    pub approval_type: String,
    pub result: String,
    pub counter: i64,
    pub id: i64,
    pub jobs: Vec<StageJob>
}

#[derive(Deserialize, Debug)]
pub struct StageJob {
    pub agent_uuid: String,
    pub name: String,
    pub scheduled_date: u64,
    pub result: String,
    pub state: String,
    pub id: i64,
    pub job_state_transitions: Vec<StageJobStateTransition>
}

#[derive(Deserialize, Debug)]
pub struct StageJobStateTransition {
    pub state_change_time: u64,
    pub id: i64,
    pub state: String
}

pub trait StageInstance {
    fn stage(&self, pipeline: &str, stage_name: &str, pipeline_counter: &str, stage_counter: &str) -> Result<Stage>;
}

impl StageInstance for Gocd {
    fn stage(&self, pipeline: &str, stage_name: &str, pipeline_counter: &str, stage_counter: &str) -> Result<Stage> {
        let data = self.get(format!("go/api/stages/{}/{}/instance/{}/{}", pipeline, stage_name, pipeline_counter, stage_counter).as_str(),
                            Some("application/json"));
        match data {
            Ok(data) => {
                let stage: Stage = serde_json::from_str(data.as_str()).unwrap();
                Ok(stage)
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
    use super::super::*;

    #[test]
    fn it_deserializes_from_example() {
        let stage: Stage = serde_json::from_str(
            include_str!("../../tests/data/stage_instance.json"))
            .unwrap();
        assert_eq!(stage.name, "defaultStage");
        assert_eq!(stage.counter, 1);
        assert_eq!(stage.jobs[0].name, "defaultJob");
        assert_eq!(stage.jobs[0].job_state_transitions[0].state, "Scheduled");
    }

    #[test]
    fn it_fetches_stage_history() {
        let _m = mock("GET", "/go/api/stages/pipeline1/stage1/instance/1/1")
            .with_status(200)
            .with_body(include_str!("../../tests/data/stage_instance.json"))
            .create();

        let gocd = Gocd::new(mockito::SERVER_URL, "foo", "bar");
        let stage = gocd.stage("pipeline1", "stage1", "1", "1").unwrap();

        assert_eq!(stage.name, "defaultStage");
    }
}

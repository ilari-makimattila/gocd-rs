extern crate serde;
extern crate serde_json;

use super::{Gocd, Result};


pub mod history;


#[derive(Deserialize, Debug)]
pub struct Pipeline {
    pub name: String,
    pub label: String,
    pub stages: Vec<PipelineStage>
}

#[derive(Deserialize, Debug)]
pub struct PipelineStage {
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct PipelineGroup {
    pub name: String,
    pub pipelines: Vec<Pipeline>
}


pub trait Pipelines {
    fn pipeline_groups(&self) -> Result<Vec<PipelineGroup>>;
}

impl Pipelines for Gocd {
    fn pipeline_groups(&self) -> Result<Vec<PipelineGroup>> {
        let data = self.get("go/api/config/pipeline_groups", 
                            Some("application/json"));
        match data {
            Ok(data) => Ok(serde_json::from_str(data.as_str()).unwrap()),
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
    fn pipeline_group_deserializes() {
        let pipeline_group: Vec<PipelineGroup> = serde_json::from_str(
            include_str!("../../tests/data/pipeline_groups.json"))
            .unwrap();
        assert_eq!(pipeline_group[0].name, "first");
        assert_eq!(pipeline_group[0].pipelines[0].name, "up42");
        assert_eq!(pipeline_group[0].pipelines[0].stages[0].name, "up42_stage");
    }

    #[test]
    fn it_fetches_pipelines() {
        let _m = mock("GET", "/go/api/config/pipeline_groups")
            .with_status(200)
            .with_body(include_str!("../../tests/data/pipeline_groups.json"))
            .create();

        let gocd = Gocd::new(mockito::SERVER_URL, "foo", "bar");
        let pipeline_groups = gocd.pipeline_groups().unwrap();

        assert_eq!(pipeline_groups[0].name, "first");
        assert_eq!(pipeline_groups[0].pipelines[0].name, "up42");
        assert_eq!(pipeline_groups[0].pipelines[0].stages[0].name, "up42_stage");
    }
}


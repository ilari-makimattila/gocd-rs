extern crate serde;
extern crate serde_json;


#[derive(Deserialize)]
struct Pipeline {
    pub name: String,
    pub label: String,
    pub stages: Vec<PipelineStage>
}

#[derive(Deserialize)]
struct PipelineStage {
    pub name: String
}

#[derive(Deserialize)]
struct PipelineGroup {
    pub name: String,
    pub pipelines: Vec<Pipeline>
}


#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;

    #[test]
    fn pipeline_group_deserializes() {
        let pipeline_group: Vec<PipelineGroup> = serde_json::from_str(
            include_str!("../../tests/data/pipeline_groups.json"))
            .unwrap();
        assert_eq!(pipeline_group[0].name, "first");
        assert_eq!(pipeline_group[0].pipelines[0].name, "up42");
        assert_eq!(pipeline_group[0].pipelines[0].stages[0].name, "up42_stage");
    }
}

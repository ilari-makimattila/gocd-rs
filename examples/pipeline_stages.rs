extern crate gocd;

use std::env;

use gocd::Gocd;
use gocd::pipelines::Pipelines;
use gocd::stages::StageInstance;
use gocd::stages::history::StageHistories;


fn main() {
    let gocd = Gocd::new(env::var("GO_URL").unwrap().as_str(),
                         env::var("GO_USERNAME").unwrap().as_str(),
                         env::var("GO_PASSWORD").unwrap().as_str());

    let pipeline_groups = gocd.pipeline_groups().unwrap();
    let pipeline = &pipeline_groups[0].pipelines[0];
    let pipeline_name = &pipeline.name;
    let stage_name = &pipeline.stages[0].name;
    println!("Getting stages for pipeline {} stage {}", pipeline_name, stage_name);
    let stage_history = gocd.stage_history(&pipeline.name, stage_name).unwrap();

    let stage = gocd
        .stage(pipeline_name, stage_name, format!("{}", stage_history[0].pipeline_counter).as_str(), stage_history[0].counter.as_str())
        .unwrap();

    println!("{:?}", stage);
}

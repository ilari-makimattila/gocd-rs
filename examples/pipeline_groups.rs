extern crate gocd;

use std::env;

use gocd::Gocd;
use gocd::pipelines::Pipelines;


fn main() {
    let gocd = Gocd::new(env::var("GO_URL").unwrap().as_str(),
                         env::var("GO_USERNAME").unwrap().as_str(),
                         env::var("GO_PASSWORD").unwrap().as_str());

    let pipelines = gocd.pipeline_groups().unwrap();

    let pipeline_group_count = pipelines.len();
    let pipeline_count = pipelines.iter().fold(0, |acc, p| acc + p.pipelines.len());
    let stage_count = pipelines.iter()
        .fold(0, |acc, pipeline| acc + pipeline.pipelines.iter().fold(0, |sa, p| sa + p.stages.len()));

    println!("groups: {}", &pipeline_group_count);
    println!("pipelines: {}", &pipeline_count);
    println!("stages: {}", &stage_count);
}

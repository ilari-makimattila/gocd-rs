extern crate gocd;

use std::env;

use gocd::Gocd;
use gocd::pipelines::Pipelines;
use gocd::pipelines::history::PipelineHistories;


fn main() {
    let gocd = Gocd::new(env::var("GO_URL").unwrap().as_str(),
                         env::var("GO_USERNAME").unwrap().as_str(),
                         env::var("GO_PASSWORD").unwrap().as_str());

    let pipeline_groups = gocd.pipeline_groups().unwrap();
    println!("Getting history for pipeline {}", &pipeline_groups[0].pipelines[0].name);
    let history = gocd.pipeline_history(pipeline_groups[0].pipelines[0].name.as_str());

    println!("{:?}", history);
}

extern crate gocd;

use std::env;

use gocd::Gocd;
use gocd::pipelines::Pipelines;


fn main() {
    let gocd = Gocd::new(env::var("GO_URL").unwrap().as_str(),
                         env::var("GO_USERNAME").unwrap().as_str(),
                         env::var("GO_PASSWORD").unwrap().as_str());

    let pipelines = gocd.pipeline_groups();

    println!("{:?}", pipelines.unwrap());
}

#[macro_use]
extern crate clap;

#[macro_use]
extern crate anyhow;

use std::collections::HashMap;

use anyhow::Result;

use pyraft_rs::raft;


fn main() -> Result<()> {
    let matches = clap_app!(run_raft => 
        (@arg ensemble: -e --ensemble +takes_value "ensemble list")
        (@arg addr: -a --addr +takes_value "ip:port[port+1")
        (@arg nid: -i --nid +takes_value "self node id")
        (@arg load: -l --load +takes_value "checkpoint filename to load")
    ).get_matches();

    if !matches.is_present("addr") {
        return Err(anyhow!("addr required"))
    }

    let addr = matches.value_of("addr").expect("no required addr argument");

    let ensemble_map: HashMap<&str, &str> = if let Some(ensembles) = matches.value_of("ensemble") {
        ensembles.split(",").map(|each_ensemble| {
            let etok = each_ensemble.split("/");
            match etok.collect::<Vec<_>>().as_slice() {
                [node_id, addr] => (node_id.to_owned(), addr.to_owned()),
                [addr] => (addr.to_owned(), addr.to_owned()),
                invalid => panic!("invalid ensemble token {:?}", invalid)
            }
        }).into_iter().collect()
    } else {
        Default::default()
    };

    let opts = raft::Opts {
        ensemble: ensemble_map,
        addr: addr,
        nid: matches.value_of("nid").unwrap_or(&addr),
        load: matches.value_of("load"),
    };

    print!("{:?}", opts);

    let mut node = raft::RaftNode::new(&opts)?;

    node.start();
    node.join();

    Ok(())
}
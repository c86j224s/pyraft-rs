#[macro_use]
extern crate clap;

use pyraft_rs::raft;

fn main() {
    let matches = clap_app!(run_raft => 
        (@arg nid: -i "self node id")
    ).get_matches();

    let node = raft::make_default_node();

    node.start();
    node.join();
}
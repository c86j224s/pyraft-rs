pub struct RaftNode;


struct Opts {
    ensemble_list: Vec<String>,
    addr: String,
    node_id: u32,
    filename_to_load_checkpoint: String,
}



pub fn make_default_node(opts: &Opts) -> RaftNode {
    RaftNode
}
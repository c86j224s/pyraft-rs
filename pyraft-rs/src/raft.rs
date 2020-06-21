use std::collections::HashMap;

use std::net::{SocketAddr, IpAddr};

use anyhow::Result;

use crate::resp;


#[derive(std::fmt::Debug)]
pub struct Opts<'a> {
    pub ensemble: HashMap<&'a str, &'a str>,
    pub addr: &'a str,
    pub nid: &'a str,
    pub load: Option<&'a str>,
}


pub struct RaftNode {
    pub nid: String,
    pub term: usize,    // TODO check type
    pub index: usize,   // TODO check type
    pub state: char,    // TODO change type to enum
    pub last_append_entry_ts: usize,    // TODO check type
    pub last_delayed_ts: usize,     // TODO check type
    pub last_checkpoint: usize,     // TODO check type
    pub first_append_entry: bool,   // TODO check type
    pub last_applied: usize,        // TODO check type
    pub commit_index: usize,        // TODO check type

    pub addr: SocketAddr,
    pub ip: IpAddr,
    pub port: u16,

    pub raft_req: resp::RespIo,
    pub raft_wait: resp::RespIo,

    pub shutdown_flag: bool,

    //pub peers: ?{},
    //pub peer_lock: ?threading.Lock(),

    pub log: RaftLog,
    pub worker: RaftWorker,

    //pub data: ?{},
    //pub data_lock: ?threading.Lock(),
    //pub ttl: ?{} ?self.data['ttl']
}

impl RaftNode {
    pub fn new(opts: &Opts) -> Result<Self> {
        let addr = opts.addr.parse()?;

        // more initialization codes...
        if let Some(_filename_to_load_checkpoint) = opts.load {
            //do load
        }

        // more initialization codes...
        //if peer == True: return

        //if self.worker != None: self.worker.set_node(self)

        //


        Ok(Self {
            nid: opts.nid.to_string(),
            term: 0,
            index: 0,
            state: 'c',
            last_append_entry_ts: 0,
            last_delayed_ts: 0,
            last_checkpoint: 0,
            first_append_entry: false,
            last_applied: 0,
            commit_index: 0,

            addr: addr,
            ip: addr.ip(),
            port: addr.port(),

            raft_req: resp::RespIo::new(None),
            raft_wait: resp::RespIo::new(None),

            shutdown_flag: false,

            //peers
            //peer_lock

            log: RaftLog::new(opts.nid),
            worker: RaftWorker,
        })
    }

    pub fn start(&mut self) {}

    pub fn join(&mut self) {}
}


pub struct RaftLog;

impl RaftLog {
    pub fn new(_nid: &str) -> Self {
        Self
    }
}


pub struct RaftWorker;
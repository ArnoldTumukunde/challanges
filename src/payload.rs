use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    #[serde(flatten)]
    pub payload: Payload,
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>
    },
    InitOk,
    Echo {
        echo: String
    },
    EchoOk {
        echo: String
    },
    Generate,
    GenerateOk{
        #[serde(rename = "id")]
        uid: String,
    },
    Broadcast{
        message: usize
    },
    BroadcastOk,
    Read,
    ReadOk{
        messages: Vec<usize>
    },
    Topology{
        topology: HashMap<String, Vec<String>>
    },
    TopologyOk,

}
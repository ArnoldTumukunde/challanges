use std::io::{StdoutLock, Write};
use uuid::Uuid;
use anyhow::{bail, Context};
use crate::payload::{Body, Message, Payload};
use crate::unique_ids::get_id;

pub struct Messages{
    message_vec: Vec<usize>
}

impl Messages {
    pub fn new() -> Messages {
        Self{
            message_vec: Vec::new()
        }
    }
}

pub struct EchoNode{
    pub messages: Vec<usize>
}

impl EchoNode {
    pub fn step(&mut self, input: Message, output: &mut StdoutLock) -> anyhow::Result<()>{  
        match input.body.payload{
            Payload::Echo { echo } =>{
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body { 
                        payload: Payload::EchoOk { echo }, 
                        id: Some(get_id()), 
                        in_reply_to: input.body.id, 
                    }
                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to echo")?;
                output.write_all(b"\n").context("write trailing new line")?;
            },
            Payload::EchoOk { .. } => {

            },
            Payload::Init{ .. } =>{
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body { 
                        payload: Payload::InitOk, 
                        id: Some(get_id()), 
                        in_reply_to: input.body.id, 
                    }
                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to init")?;
                output.write_all(b"\n").context("write trailing new line")?;
            },
            Payload::InitOk  => bail!("received init ok method"),
            Payload::Generate => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body { 
                        payload: Payload::GenerateOk{uid: Uuid::new_v4().to_string()}, 
                        id: Some(get_id()), 
                        in_reply_to: input.body.id, 
                    }
                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to generate")?;
                output.write_all(b"\n").context("write trailing new line")?;
            },
            Payload::GenerateOk{..}  => bail!("received generate ok method"),
            Payload::Broadcast{ message } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body { 
                        payload: Payload::BroadcastOk, 
                        id: Some(get_id()), 
                        in_reply_to: input.body.id, 
                    }
                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to broadcast")?;
                self.messages.push(message);
                output.write_all(b"\n").context("write trailing new line")?;    
            },
            Payload::BroadcastOk => bail!("received generate broadcast_ok method"),
            Payload::Read => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body { 
                        payload: Payload::ReadOk{ messages: self.messages.clone() }, 
                        id: Some(get_id()), 
                        in_reply_to: input.body.id, 
                    }
                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to read")?;
                output.write_all(b"\n").context("write trailing new line")?;  
            },
            Payload::ReadOk{ .. } => bail!("received generate broadcast_ok method"),
            Payload::Topology{ topology } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body { 
                        payload: Payload::TopologyOk, 
                        id: Some(get_id()), 
                        in_reply_to: input.body.id, 
                    }
                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to topology")?;
                output.write_all(b"\n").context("write trailing new line")?; 
            },
            Payload::TopologyOk => bail!("received generate broadcast_ok method"),
        }

        Ok(())
    }
}

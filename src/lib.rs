use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::{BufRead, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message<T> {
    pub src: String,
    pub dest: String,
    pub body: MessageBody<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageBody<T> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: T,
}

pub fn into_reply<T>(incoming_msg: &Message<T>, payload: T) -> Message<T> {
    Message {
        src: incoming_msg.dest.clone(),
        dest: incoming_msg.src.clone(),
        body: MessageBody {
            msg_id: incoming_msg.body.msg_id,
            in_reply_to: incoming_msg.body.msg_id,
            payload,
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum InitType {
    Init,
    InitOk,
}
#[derive(Debug, Serialize, Deserialize)]
struct Init {
    #[serde(rename = "type")]
    typ: InitType,
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct InitOk {
    #[serde(rename = "type")]
    typ: InitType,
}

impl InitOk {
    fn new() -> Self {
        Self {
            typ: InitType::InitOk,
        }
    }

    fn from(init_msg: Message<Init>) -> Message<InitOk> {
        Message {
            src: init_msg.dest,
            dest: init_msg.src,
            body: MessageBody {
                msg_id: Some(0),
                in_reply_to: init_msg.body.msg_id,
                payload: InitOk::new(),
            },
        }
    }
}

pub trait Node<P> {
    fn handle(self, message: &Message<P>) -> anyhow::Result<Option<Message<P>>>;
}

pub fn main_loop<P, N: Node<P> + Copy>(node: N) -> anyhow::Result<()>
where
    P: DeserializeOwned + Serialize,
{
    let stdin = std::io::stdin().lock();
    let mut stdin = stdin.lines();
    let mut stdout = std::io::stdout().lock();

    let init_msg: Message<Init> = serde_json::from_str(
        &stdin
            .next()
            .expect("no init message received")
            .context("failed to read init message from stdin")?,
    )
    .context("init message could not be deserialized")?;

    let reply = InitOk::from(init_msg);
    serde_json::to_writer(&mut stdout, &reply).context("serialize response to init")?;
    // Write the response for an init message
    stdout.write_all(b"\n").context("write trailing newline")?;

    for line in stdin {
        let line = line.context("Maelstrom input from STDIN could not be read")?;
        let message: Message<P> = serde_json::from_str(&line)
            .context("Maelstrom input from STDIN could not be deserialized")?;

        let reply: Option<Message<P>> = node
            .handle(&message)
            .context("Node handle function failed")?;

        match reply {
            Some(msg) => {
                serde_json::to_writer(&mut stdout, &msg).context("serialize response")?;
                stdout.write_all(b"\n").context("write trailing newline")?;
            }
            None => {}
        }
    }

    Ok(())
}

use rust_dist_sys::{main_loop, Message, Node};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum EchoPayload {
    Echo { echo: String },
    EchoOk { echo: String },
}
#[derive(Copy, Clone)]
struct EchoNode {
    id: usize,
}

impl Node<EchoPayload> for EchoNode {
    fn handle(
        &mut self,
        message: &mut Message<EchoPayload>,
    ) -> anyhow::Result<Option<Message<EchoPayload>>> {
        match &message.body.payload {
            EchoPayload::Echo { echo } => {
                let reply = message.clone().into_reply(EchoPayload::EchoOk {
                    echo: echo.to_string(),
                });

                anyhow::Ok(Some(reply))
            }
            EchoPayload::EchoOk { .. } => anyhow::Ok(None),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let mut node = EchoNode { id: 1 };
    main_loop(&mut node)
}

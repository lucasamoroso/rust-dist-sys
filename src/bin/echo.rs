use rust_dist_sys::{into_reply, main_loop, Message, Node};
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
        self,
        message: &Message<EchoPayload>,
    ) -> anyhow::Result<Option<Message<EchoPayload>>> {
        match &message.body.payload {
            EchoPayload::Echo { echo } => {
                let reply = into_reply(
                    &message,
                    EchoPayload::EchoOk {
                        echo: echo.to_string(),
                    },
                );

                anyhow::Ok(Some(reply))
            }
            EchoPayload::EchoOk { .. } => anyhow::Ok(None),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let node = EchoNode { id: 1 };
    main_loop(node)
}

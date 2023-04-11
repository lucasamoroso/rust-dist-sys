use rust_dist_sys::{into_reply, main_loop, Message, Node};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum UniqueIdPayload {
    Generate,
    GenerateOk { id: Uuid },
}
#[derive(Copy, Clone)]
struct UniqueIdNode {
    id: usize,
}

impl Node<UniqueIdPayload> for UniqueIdNode {
    fn handle(
        self,
        message: &Message<UniqueIdPayload>,
    ) -> anyhow::Result<Option<Message<UniqueIdPayload>>> {
        match &message.body.payload {
            UniqueIdPayload::Generate => {
                let reply =
                    into_reply(&message, UniqueIdPayload::GenerateOk { id: Uuid::new_v4() });

                anyhow::Ok(Some(reply))
            }
            UniqueIdPayload::GenerateOk { .. } => anyhow::Ok(None),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let node = UniqueIdNode { id: 1 };
    main_loop(node)
}

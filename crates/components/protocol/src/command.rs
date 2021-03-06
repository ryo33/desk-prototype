use crate::EntranceCommand;

use super::primitives::*;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "c")] // stands for code
pub enum Command {
    Login(Login),
    CreateRoom {
        room_name: RoomName,
    },
    CreatePublicRoom {
        room_name: RoomName,
    },
    EntranceCommand(EntranceCommand),
    /// Update the topic with the contents
    Update {
        room_id: RoomId,
        topic: Topic,
        all_contents: Vec<Bytes>,
    },
    /// Push a content into the topic
    Push {
        room_id: RoomId,
        topic: Topic,
        contents: Bytes,
    },
    /// Listen to the topic
    Listen {
        room_id: RoomId,
        local_user_id: RoomLocalUserId,
        topic: Topic,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)] // No Debug to avoid expose a credential in debug log!
pub struct Login {
    pub token: Token,
}

impl std::fmt::Debug for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Login").field("token", &"****").finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn debug_of_login_does_not_contain_credential() {
        let target = Command::Login(Login {
            token: vec![1, 2, 3].into(),
        });
        assert_eq!(format!("{:?}", target), r#"Login(Login { token: "****" })"#)
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Commands(Vec<Command>);
impl Commands {
    pub fn iter(&self) -> impl Iterator<Item = &Command> {
        self.0.iter()
    }

    pub fn push(&mut self, event: Command) {
        self.0.push(event);
    }

    pub fn clear(&mut self) {
        self.0.clear();
        self.0.truncate(32);
    }
}

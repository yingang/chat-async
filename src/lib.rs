use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ClientMsg {
    JoinGroup { group_name: String },
    ExitGroup { group_name: String },
    GroupMessage { group_name: String, message: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ServerMsg {
    GroupMessage {
        group_name: String,
        sender: String,
        message: String,
    },
    ErrorMessage {
        message: String,
    },
}

#[cfg(test)] // 注意不是tests
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let join = ClientMsg::JoinGroup {
            group_name: "TESTGROUP".into(),
        };
        println!("{:?}", join);
        let json = serde_json::to_string(&join).unwrap();
        println!("{}", json);
        let msg: ClientMsg = serde_json::from_str(&json).unwrap();
        println!("{:?}", msg);
        assert_eq!(msg, join);
    }
}

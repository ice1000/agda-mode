use crate::resp::{Resp, Status};

#[test]
fn simple_status_de() {
    let a = Status::default();
    let json = serde_json::to_string(&a).unwrap();
    println!("{}", json);
}

#[test]
fn simple_resp_de() {
    let a = Resp::Status {
        status: Default::default(),
    };
    let json = serde_json::to_string(&a).unwrap();
    println!("{}", json);
}

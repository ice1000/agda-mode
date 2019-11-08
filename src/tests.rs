use crate::resp::Status;

#[test]
fn simple_status_de() {
    let a = Status::default();
    let json = serde_json::to_string(&a).unwrap();
    println!("{}", json);
}

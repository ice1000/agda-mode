use crate::resp::{MakeCaseVariant, Resp};

#[test]
fn simple_running_info_de() {
    let json = serde_json::to_string(&Resp::ClearRunningInfo).unwrap();
    println!("{}", json);
}

#[test]
fn simple_make_case_de() {
    let a = Resp::MakeCase {
        variant: MakeCaseVariant::Function,
        clauses: vec!["f a = a".to_owned()],
        interaction_point: 233,
    };
    let json = serde_json::to_string(&a).unwrap();
    println!("{}", json);
}

#[test]
fn simple_resp_status_de() {
    let a = Resp::Status {
        status: Default::default(),
    };
    let json = serde_json::to_string(&a).unwrap();
    println!("{}", json);
}

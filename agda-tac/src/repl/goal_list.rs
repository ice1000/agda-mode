use crate::file_io::{Monad, Repl};
use agda_mode::agda::{preprint_agda_result, ReplState};

pub async fn reload_unit(agda: &mut Repl) -> Monad {
    reload(agda).await.map(|_| ())
}

pub async fn reload(agda: &mut Repl) -> Monad<bool> {
    let da = &mut agda.agda;
    da.reload_file().await?;
    poll_goals(da).await
}

pub async fn poll_goals(agda: &mut ReplState) -> Monad<bool> {
    if let Some(agw) = preprint_agda_result(agda.next_all_goals_warnings().await?) {
        if agw.visible_goals.is_empty() {
            println!("No goals.");
        } else {
            println!("Goals:");
        }
        for goal in agw.visible_goals {
            // I believe `goal` will always be `OfType`.
            match goal.try_as_of_type() {
                Ok(ok) => println!("?{} : {}", ok.constraint_obj, ok.of_type),
                Err(bad) => eprintln!("[WARN]: unexpected goal: {:?}", bad),
            }
        }
        if !agw.invisible_goals.is_empty() {
            println!("Unsolved metas:");
        }
        for meta in agw.invisible_goals {
            println!("{}", meta);
        }
        agda.next_goals().await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

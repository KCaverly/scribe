use crate::NOTES_DIR;
use std::process::{Command, Stdio};

pub struct Git {}

impl Git {
    fn pull() -> bool {
        let pull_cmd = Command::new("git")
            .arg("pull")
            .current_dir(&*NOTES_DIR)
            .stdout(Stdio::null())
            .status()
            .unwrap();
        return pull_cmd.success();
    }

    fn add() -> bool {
        let add_cmd = Command::new("git")
            .args(vec!["add", "."])
            .current_dir(&*NOTES_DIR)
            .stdout(Stdio::null())
            .status()
            .unwrap();
        return add_cmd.success();
    }

    fn commit(commit_message: Option<String>) -> bool {
        let msg: String;
        if commit_message.is_none() {
            msg = "scribe: Syncing".to_string();
        } else {
            msg = commit_message.unwrap();
        }

        let commit_cmd = Command::new("git")
            .args(vec!["commit", "-m", &msg])
            .stdout(Stdio::null())
            .status()
            .unwrap();

        return commit_cmd.success();
    }

    fn push() -> bool {
        let push_cmd = Command::new("git")
            .arg("push")
            .current_dir(&*NOTES_DIR)
            .stdout(Stdio::null())
            .status()
            .unwrap();
        return push_cmd.success();
    }

    pub fn sync(commit_message: Option<String>) -> bool {
        if Self::pull() {
            if Self::add() {
                if Self::commit(commit_message) {
                    return Self::push();
                }
            }
        }
        return false;
    }
}

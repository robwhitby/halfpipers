use std::fmt;

#[derive(Clone)]
pub struct Env {
    pub path: String,
    pub git_root: String,
}

impl Env {
    pub fn new() -> Self {
        Self {
            path: "".to_string(),
            git_root: "".to_string(),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "env: path={} git_root={}", self.path, self.git_root)
    }
}

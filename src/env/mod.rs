use std::fmt;

#[derive(Clone)]
pub struct Env {
    path: String,
    git_root: String,
}

impl Env {
    pub fn new() -> Self {
        Self {
            path: "app".to_string(),
            git_root: "/my/monorepo".to_string(),
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "env: path={} git_root={}", self.path, self.git_root)
    }
}

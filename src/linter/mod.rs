mod rules;

use crate::linter::rules::*;
use crate::{Env, Manifest};

pub struct Linter {
    rules: Rules,
}

pub type Issues = Vec<Issue>;

impl Linter {
    pub fn new() -> Self {
        Self::with_rules(vec![team_must_not_contain_spaces, pipeline_should_be_lowercase])
    }

    pub fn with_rules(rules: Rules) -> Self {
        Self { rules }
    }

    pub fn lint(&self, env: &Env, manifest: &Manifest) -> Issues {
        self.rules.iter().flat_map(|r| r(env, manifest)).collect()
    }
}

pub trait ContainsError {
    fn contains_error(&self) -> bool;
}

impl ContainsError for Issues {
    fn contains_error(&self) -> bool {
        self.iter().any(|i| matches!(i, Issue::Error { .. }))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Issue {
    Warning(String),
    Error(String),
}

impl Issue {
    pub fn error(s: &str) -> Issue {
        Self::Error(s.to_string())
    }
    pub fn warning(s: &str) -> Issue {
        Self::Warning(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_rules() {
        let rule1: Rule = |env, man| Some(Issue::error(&format!("{} {}", env.path, man.pipeline)));
        let rule2: Rule = |_, _| Some(Issue::warning("rule2"));
        let rule3: Rule = |_, _| None;
        let rules = vec![rule1, rule2, rule3];

        let env = Env {
            path: "/path".to_string(),
            ..Env::new()
        };

        let manifest = Manifest {
            pipeline: "pipe".to_string(),
            ..Manifest::new()
        };

        let issues = Linter::with_rules(rules).lint(&env, &manifest);

        assert_eq!(issues.len(), 2);
        assert_eq!(issues.get(0).unwrap(), &Issue::error("/path pipe"));
        assert_eq!(issues.get(1).unwrap(), &Issue::warning("rule2"));
    }

    #[test]
    fn contains_error() {
        assert!(vec![Issue::warning(""), Issue::error("")].contains_error());
        assert!(!vec![Issue::warning("")].contains_error());
        assert!(!vec![].contains_error());
    }
}

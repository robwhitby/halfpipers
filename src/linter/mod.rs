mod rules;

use crate::linter::rules::*;
use crate::{Env, Manifest};

pub struct Linter {
    env: Env,
    rules: Vec<Rule>,
}

impl Linter {
    pub fn new(env: &Env) -> Self {
        Self::with_rules(env, vec![team_must_not_contain_spaces, pipeline_should_be_lowercase])
    }

    pub fn with_rules(env: &Env, rules: Vec<Rule>) -> Self {
        Self { env: env.clone(), rules }
    }

    pub fn lint(&self, manifest: &Manifest) -> Vec<Issue> {
        self.rules.iter().flat_map(|r| r(&self.env, manifest)).collect()
    }
}

pub trait ContainsError {
    fn contains_error(&self) -> bool;
}

impl ContainsError for Vec<Issue> {
    fn contains_error(&self) -> bool {
        self.iter().any(|i| matches!(i, Issue::Error { .. }))
    }
}

#[derive(Debug, PartialEq)]
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
        let rule1: Rule = |_, _| Some(Issue::error("rule1"));
        let rule2: Rule = |_, _| Some(Issue::warning("rule2"));
        let rule3: Rule = |_, _| None;

        let linter = Linter::with_rules(&Env::new(), vec![rule1, rule2, rule3]);

        let manifest = Manifest {
            pipeline: "Pipeline".to_string(),
            team: "team name".to_string(),
            tasks: vec![],
        };

        let issues = linter.lint(&manifest);

        assert_eq!(issues.len(), 2);
        assert_eq!(issues.get(0).unwrap(), &Issue::error("rule1"));
        assert_eq!(issues.get(1).unwrap(), &Issue::warning("rule2"));
    }

    #[test]
    fn contains_error() {
        assert!(vec![Issue::warning(""), Issue::error("")].contains_error());
        assert!(!vec![Issue::warning("")].contains_error());
        assert!(!vec![].contains_error());
    }
}

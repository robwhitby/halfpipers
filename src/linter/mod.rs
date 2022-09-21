mod rules;

use crate::linter::rules::*;
use crate::Manifest;

pub trait Linterer {
    fn lint(&self, manifest: &Manifest) -> Vec<Issue>;
}

pub struct Linter {
    rules: Vec<Rule>,
}

impl Linter {
    pub fn new() -> Self {
        Self::with_rules(vec![team_must_not_contain_spaces, pipeline_should_be_lowercase])
    }

    pub fn with_rules(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}

impl Linterer for Linter {
    fn lint(&self, manifest: &Manifest) -> Vec<Issue> {
        self.rules.iter().flat_map(|r| r(manifest)).collect()
    }
}

pub trait ContainsError {
    fn contains_error(&self) -> bool;
}

impl ContainsError for Vec<Issue> {
    fn contains_error(&self) -> bool {
        self.iter().any(|i| match i {
            Issue::Error(_) => true,
            _ => false,
        })
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
    fn happy() {
        let rule1: Rule = |_| Some(Issue::error("rule1"));
        let rule2: Rule = |_| Some(Issue::warning("rule2"));
        let rule3: Rule = |_| None;

        let linter = Linter::with_rules(vec![rule1, rule2, rule3]);

        let manifest = Manifest {
            pipeline: "Pipeline".to_string(),
            team: "team name".to_string(),
            tasks: vec![],
        };

        let issues = linter.lint(&manifest);

        assert_eq!(issues.len(), 2);
        assert!(issues.contains_error());
        assert_eq!(issues.get(0).unwrap(), &Issue::error("rule1"));
        assert_eq!(issues.get(1).unwrap(), &Issue::warning("rule2"));
    }
}

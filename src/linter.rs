use crate::lint_rules::*;
use crate::Manifest;

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

    pub fn lint(&self, manifest: &Manifest) -> Vec<Issue> {
        self.rules.iter().flat_map(|r| r(manifest)).collect()
    }
}

#[derive(Debug, PartialEq)]
pub enum Issue {
    Warning(String),
    Error(String),
}

pub fn contains_error(issues: &Vec<Issue>) -> bool {
    issues.iter().any(|i| match i {
        Issue::Error(_) => true,
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy() {
        let rule1: Rule = |_| Some(Issue::Error("rule1".to_string()));
        let rule2: Rule = |_| Some(Issue::Warning("rule2".to_string()));
        let rule3: Rule = |_| None;

        let linter = Linter::with_rules(vec![rule1, rule2, rule3]);

        let manifest = Manifest {
            pipeline: "Pipeline".to_string(),
            team: "team name".to_string(),
            tasks: vec![],
        };

        let issues = linter.lint(&manifest);

        assert_eq!(issues.len(), 2);
        assert_eq!(issues.get(0).unwrap(), &Issue::Error("rule1".to_string()));
        assert_eq!(issues.get(1).unwrap(), &Issue::Warning("rule2".to_string()));
    }
}

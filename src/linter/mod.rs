mod issue;
mod rules;

use crate::linter::rules::*;
use crate::{Env, Manifest};
pub use issue::{ContainsError, Issue, Issues};

pub struct Linter {
    rules: Rules,
}

pub trait Lint {
    fn lint(&self, env: &Env, manifest: &Manifest) -> Issues;
}

impl Linter {
    pub fn new() -> Self {
        Self::with_rules(vec![team_must_not_contain_spaces, pipeline_should_be_lowercase])
    }

    pub fn with_rules(rules: Rules) -> Self {
        Self { rules }
    }
}

impl Lint for Linter {
    fn lint(&self, env: &Env, manifest: &Manifest) -> Issues {
        self.rules.iter().flat_map(|r| r(env, manifest)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linter::rules::Rule;
    use issue::*;

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
}

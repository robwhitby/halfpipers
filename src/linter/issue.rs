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

pub type Issues = Vec<Issue>;

pub trait ContainsError {
    fn contains_error(&self) -> bool;
}

impl ContainsError for Issues {
    fn contains_error(&self) -> bool {
        self.iter().any(|i| matches!(i, Issue::Error { .. }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_error() {
        assert!(vec![Issue::warning(""), Issue::error("")].contains_error());
        assert!(!vec![Issue::warning("")].contains_error());
        assert!(!vec![].contains_error());
    }
}

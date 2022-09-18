use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Manifest {
    pub pipeline: String,
    pub team: String,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Task {
    Run {
        #[serde(flatten)]
        common: CommonTask,
        script: String,
    },
    DockerCompose {
        #[serde(flatten)]
        common: CommonTask,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        compose_file: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct CommonTask {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    retries: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timeout: Option<String>,
}

impl Manifest {
    pub fn from_yaml(raw_manifest: &String) -> Result<Manifest, serde_yaml::Error> {
        let manifest: Manifest = serde_yaml::from_str(&raw_manifest)?;
        Ok(manifest)
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        let yaml = serde_yaml::to_string(self)?;
        Ok(yaml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy() {
        let input = "
        pipeline: my-pipe
        team: my-team
        tasks: 
        - type: run
          name: build
          script: ./build
        - type: run
          script: ./test
          name: ''
        - type: docker-compose
        ";

        let expected = Manifest {
            pipeline: "my-pipe".to_string(),
            team: "my-team".to_string(),
            tasks: vec![
                Task::Run {
                    common: CommonTask {
                        name: Some("build".to_string()),
                        ..Default::default()
                    },
                    script: "./build".to_string(),
                },
                Task::Run {
                    common: CommonTask {
                        name: Some("".to_string()),
                        ..Default::default()
                    },
                    script: "./test".to_string(),
                },
                Task::DockerCompose {
                    common: Default::default(),
                    compose_file: None,
                },
            ],
        };

        assert_eq!(expected, Manifest::from_yaml(&input.to_string()).unwrap());
    }

    #[test]
    fn sad_yaml() {
        let input = String::from("some rubbish");

        let err = Manifest::from_yaml(&input).unwrap_err();
        assert!(err.to_string().contains("invalid type"));
    }

    #[test]
    fn missing_field() {
        let input = "
        pipeline: my-pipe
        ";

        let err = Manifest::from_yaml(&input.to_string()).unwrap_err();
        assert!(err.to_string().contains("missing field `team`"));
    }
}

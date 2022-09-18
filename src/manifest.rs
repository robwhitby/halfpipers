use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Manifest {
    pub pipeline: String,
    pub team: String,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum Task {
    Run {
        #[serde(default)]
        name: String,
        script: String,
    },
    DockerCompose {
        #[serde(default)]
        name: String,
        #[serde(default)]
        compose_file: String,
    },
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
        - type: docker-compose
        ";

        let expected = Manifest {
            pipeline: "my-pipe".to_string(),
            team: "my-team".to_string(),
            tasks: vec![
                Task::Run {
                    name: "build".to_string(),
                    script: "./build".to_string(),
                },
                Task::Run {
                    name: "".to_string(),
                    script: "./test".to_string(),
                },
                Task::DockerCompose {
                    name: "".to_string(),
                    compose_file: "".to_string(),
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

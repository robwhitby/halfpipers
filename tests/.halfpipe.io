pipeline: Pipeline-name
team: Halfpipe-team
tasks:
- type: run
  name: my run task
  script: ./run.sh
  timeout: 1h2m

- type: run
  script: ./test.sh

- type: docker-compose


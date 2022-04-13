# Code Challenge ![workflow](https://github.com/itsbalamurali/code-challenge/actions/workflows/ci-cd.yml/badge.svg)


Repository contains a simple rust webserver along with terraform and helm charts used for deployment.

[Github Actions Workflow](./.github/workflows/ci-cd.yml) contains various jobs to test, build and deploy the webserver to kubernetes via terraform using the helm chart.

| Directory | Description |
|----|----|
| `src`| Rust code for our webserver.|
|`helm`| kubernetes helm chart.|
|`deploy`| Terraform to deploy using helm chart.|
|`.github`| Github Actions workflows.|

## Development setup
The following are pre-requisties
- Rust - [Can be installed via Rust Up](https://rustup.rs)
- Make
- Docker
- Helm
- Terraform
- Kubernetes

## Building
You can make use of `Makefile`.

```bash
make all #you can run make test / make build.
```
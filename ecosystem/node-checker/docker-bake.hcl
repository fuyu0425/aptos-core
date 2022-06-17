# This is a docker bake file in HCL syntax.
# It provides a high-level mechenanism to build multiple dockerfiles in one shot.
# Check https://crazymax.dev/docker-allhands2-buildx-bake and https://docs.docker.com/engine/reference/commandline/buildx_bake/#file-definition for an intro.

variable "GIT_SHA" {}
variable "GIT_BRANCH" {}
variable "AWS_ECR_ACCOUNT_NUM" {}
variable "GCP_DOCKER_ARTIFACT_REPO" {}
variable "ecr_base" {
  default = "${AWS_ECR_ACCOUNT_NUM}.dkr.ecr.us-west-2.amazonaws.com/aptos"
}
variable "normalized_git_branch" {
  default = regex_replace("${GIT_BRANCH}", "[^a-zA-Z0-9]", "-")
}

group "default" {
  targets = [
    "node-checker",
  ]
}

target "node-checker" {
  dockerfile = "Dockerfile"
  context    = "."
  cache-from = [
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/node-checker:cache-main",
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/node-checker:cache-auto",
    "type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/node-checker:cache-${normalized_git_branch}",
  ]
  cache-to = ["type=registry,ref=${GCP_DOCKER_ARTIFACT_REPO}/node-checker:cache-${normalized_git_branch},mode=max"]
  tags = [
    "${ecr_base}/node-checker:${GIT_SHA}",
    "${GCP_DOCKER_ARTIFACT_REPO}/node-checker:${GIT_SHA}",
  ]
}

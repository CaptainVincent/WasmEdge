group "default" {
  targets = ["cross"]
}

target "base" {
  dockerfile = "./utils/docker/Dockerfile.debian-dynamic"
  context    = "."
  output     = ["build"]
}

target "cross" {
  inherits  = ["base"]
  platforms = [
    "linux/amd64",
    "linux/arm64"
  ]
}

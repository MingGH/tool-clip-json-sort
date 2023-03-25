pipeline {
  agent any

  stages {

    stage('Build for Linux') {
      steps {
        // 使用 Cross 工具链构建 Linux 版本的可执行文件
        sh 'docker run --rm -v "$(pwd)":/workspace -w /workspace -u $(id -u):$(id -g) rust:latest sh -c "cargo install cross && cross build --release --target x86_64-unknown-linux-musl"'
        // 将构建结果复制到 Jenkins 工作空间
        sh 'mkdir -p dist/linux && cp target/x86_64-unknown-linux-musl/release/tool-clip-json-sort dist/linux'
      }
    }

    stage('Build for macOS') {
      steps {
        // 使用 Cross 工具链构建 macOS 版本的可执行文件
        sh 'docker run --rm -v "$(pwd)":/workspace -w /workspace -u $(id -u):$(id -g) rust:latest sh -c "cargo install cross && cross build --release --target x86_64-apple-darwin"'
        // 将构建结果复制到 Jenkins 工作空间
        sh 'mkdir -p dist/macos && cp target/x86_64-apple-darwin/release/tool-clip-json-sort dist/macos'
      }
    }

    stage('Build for Windows') {
      steps {
        // 使用 Cross 工具链构建 Windows 版本的可执行文件
        sh 'docker run --rm -v "$(pwd)":/workspace -w /workspace -u $(id -u):$(id -g) rust:latest sh -c "cargo install cross && cross build --release --target x86_64-pc-windows-gnu"'
        // 将构建结果复制到 Jenkins 工作空间
        sh 'mkdir -p dist/windows && cp target/x86_64-pc-windows-gnu/release/tool-clip-json-sort.exe dist/windows'
      }
    }

    stage('Package') {
      steps {
        // 将构建结果打包为 zip 文件
        sh 'zip -r dist.zip dist'
        // 将构建结果上传到 Jenkins 服务器上的 Artifacts 目录
        archiveArtifacts artifacts: 'dist.zip', onlyIfSuccessful: true
      }
    }
  }
}

pipeline {
  agent any

  stages {
    stage ("Clippy") {
      steps {
        sh 'cargo clippy'
      }
    }

    stage ("Format check") {
      steps {
        sh 'cargo fmt --check'
      }
    }

    stage ("Test") {
      steps {
        sh 'cargo test --no-fail-fast -- --test-threads=1'
      }
    }

    stage ("Test (Release Mode)") {
      steps {
        sh 'cargo test --release --no-fail-fast'
      }
    }
  }

  post { 
    always { 
      cleanWs()
    }
  }
}

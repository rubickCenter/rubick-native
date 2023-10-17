const { readFileSync, writeFileSync } = require("fs")
const path = require("path");
const { version } = require("../package.json")
const pkg = require("./package.json")
pkg.version = version
writeFileSync("./package.json", JSON.stringify(pkg))

const ijs_path = path.join(__dirname, "index.js")
const ijs = readFileSync(ijs_path, "utf-8")
const pkg_name = pkg.name
const mijs = `const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'win32':
    switch (arch) {
      case 'x64':
        try {
          nativeBinding = require('./${pkg_name}.win32-x64-msvc.node')
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        try {
          nativeBinding = require('./${pkg_name}.win32-ia32-msvc.node')
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        try {
          nativeBinding = require('./${pkg_name}.win32-arm64-msvc.node')
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(\`Unsupported architecture on Windows: \${arch}\`)
    }
    break
  case 'darwin':
    try {
      nativeBinding = require('./${pkg_name}.darwin-universal.node')
      break
    } catch { }
    switch (arch) {
      case 'x64':
        try {
          nativeBinding = require('./${pkg_name}.darwin-x64.node')
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        try {
          nativeBinding = require('./${pkg_name}.darwin-arm64.node')
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(\`Unsupported architecture on macOS: \${arch}\`)
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          try {
            nativeBinding = require('./${pkg_name}.linux-x64-musl.node')
          } catch (e) {
            loadError = e
          }
        } else {
          try {
            nativeBinding = require('./${pkg_name}.linux-x64-gnu.node')
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          try {
            nativeBinding = require('./${pkg_name}.linux-arm64-musl.node')
          } catch (e) {
            loadError = e
          }
        } else {
          try {
            nativeBinding = require('./${pkg_name}.linux-arm64-gnu.node')
          } catch (e) {
            loadError = e
          }
        }
        break
      default:
        throw new Error(\`Unsupported architecture on Linux: \${arch}\`)
    }
    break
  default:
    throw new Error(\`Unsupported OS: \${platform}, architecture: \${arch}\`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(\`Failed to load native binding\`)
}
`

const nijs = mijs + ijs.split(`throw new Error(\`Failed to load native binding\`)\n}\n`).pop()

writeFileSync(ijs_path, nijs)
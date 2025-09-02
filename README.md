# Zed MATLAB LSP Extension

A Zed extension that enables MATLAB Language Server Protocol (LSP) support for enhanced MATLAB development experience in the Zed editor.

## Features

- **Syntax Highlighting**: Full MATLAB syntax highlighting and language detection
- **Language Server Integration**: Seamless integration with MATLAB Language Server
- **Intelligent Code Analysis**: Hover information, completion, and diagnostics
- **Cross-Platform Support**: Works on macOS and Linux

## Prerequisites

- **MATLAB**: R2021b or later (tested with R2025a)
- **Node.js**: Version 16 or later
- **Zed Editor**: Version 0.201 or later
- **Git**: For cloning repositories

## Installation

### Step 1: Install the Zed Extension

1. Clone this repository or install via Zed's extension marketplace (if published)
2. If installing manually, place the extension files in your Zed extensions directory

### Step 2: Build MATLAB Language Server

The MATLAB Language Server must be built separately as it's not distributed as a binary.

1. **Clone the MATLAB Language Server repository:**
   ```bash
   git clone https://github.com/mathworks/MATLAB-language-server.git
   cd MATLAB-language-server
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Build the language server:**
   ```bash
   npm run compile
   ```

4. **Verify the build:**
   After successful compilation, you should have an `out/index.js` file:
   ```bash
   ls -la out/index.js
   node out/index.js --help
   ```

### Step 3: Configure Zed Settings

Add the following configuration to your Zed `settings.json` file. You can access this via `Zed > Settings > Open Settings` or `Cmd+,` on macOS.

#### Recommended Configuration (Absolute Paths)

```json
{
  "lsp": {
    "matlab-language-server": {
      "binary": {
        "path": "/path/to/your/node",
        "arguments": [
          "/path/to/MATLAB-language-server/out/index.js",
          "--stdio",
          "--matlabInstallPath", "/path/to/your/MATLAB/installation",
          "--matlabConnectionTiming", "onStart"
        ],
        "env": {
          "PATH": "/path/to/your/MATLAB/installation/bin:/usr/bin:/bin:/usr/sbin:/sbin"
        }
      }
    }
  }
}
```

#### Example Configuration (macOS with Homebrew)

```json
{
  "lsp": {
    "matlab-language-server": {
      "binary": {
        "path": "/opt/homebrew/opt/node@22/bin/node",
        "arguments": [
          "/Users/username/MATLAB-language-server/out/index.js",
          "--stdio",
          "--matlabInstallPath", "/Applications/MATLAB_R2025a.app",
          "--matlabConnectionTiming", "onStart"
        ],
        "env": {
          "PATH": "/Applications/MATLAB_R2025a.app/bin:/usr/bin:/bin:/usr/sbin:/sbin"
        }
      }
    }
  }
}
```

#### Alternative Configuration (Using Command Names)

If you prefer to use command names instead of absolute paths:

```json
{
  "lsp": {
    "matlab-language-server": {
      "binary": {
        "path": "node",
        "arguments": [
          "/Users/username/MATLAB-language-server/out/index.js",
          "--stdio",
          "--matlabInstallPath", "/Applications/MATLAB_R2025a.app"
        ],
        "env": {
          "PATH": "/Applications/MATLAB_R2025a.app/bin:/opt/homebrew/opt/node@22/bin:/usr/bin:/bin:/usr/sbin:/sbin"
        }
      }
    }
  }
}
```

### Configuration Notes

- **Argument Separation**: The `--matlabInstallPath` option and its value must be separate array elements
- **PATH Environment**: Include MATLAB's bin directory and system paths, but avoid including the matlab binary file directly
- **Node.js Path**: Use absolute paths for Node.js to avoid PATH resolution issues

## Platform-Specific Setup

### macOS

```bash
# Example paths for macOS
MATLAB_PATH="/Applications/MATLAB_R2025a.app"
NODE_PATH="/opt/homebrew/opt/node@22/bin/node"  # or "/usr/local/bin/node"
```

### Linux

```bash
# Example paths for Linux
MATLAB_PATH="/usr/local/MATLAB/R2025a"  # or "/opt/MATLAB/R2025a"
NODE_PATH="/usr/bin/node"
```

## Verification

### 1. Test MATLAB Installation
```bash
# macOS
/Applications/MATLAB_R2025a.app/bin/matlab -batch "disp(version); exit"

# Linux
/usr/local/MATLAB/R2025a/bin/matlab -batch "disp(version); exit"
```

### 2. Test Language Server
```bash
node /path/to/MATLAB-language-server/out/index.js --help
```

### 3. Verify in Zed
1. Open a `.m` file in Zed
2. Check **View > Extension Logs** for:
   - Correct binary path and arguments
   - `matlabls: Launching MATLAB...` message
   - Successful initialization without errors

## Troubleshooting

### Common Issues

| Error | Cause | Solution |
|-------|-------|----------|
| `spawn matlab ENOENT` | MATLAB not in PATH | Add MATLAB bin directory to PATH or use `--matlabInstallPath` |
| MATLAB exits immediately | Missing system paths | Add `/usr/bin:/bin:/usr/sbin:/sbin` to PATH |
| Arguments not parsed | Incorrect argument format | Separate `--matlabInstallPath` and its value into different array elements |
| Language server not starting | Node.js path issues | Use absolute path for Node.js binary |

### Environment Variables

You can also use environment variables for configuration:

```bash
export MATLAB_LSP_SERVER="/path/to/MATLAB-language-server/out/index.js"
export MATLAB_LSP_NODE="/path/to/node"
export MATLAB_INSTALL_PATH="/Applications/MATLAB_R2025a.app"
```

## Development

### Building the Extension

This extension is built using Rust and compiled to WebAssembly:

```bash
# Add WASI target
rustup target add wasm32-wasip1

# Build the extension
cargo build --release --target wasm32-wasip1

# Copy the built WASM file
cp target/wasm32-wasip1/release/*.wasm extension.wasm
```

### Testing

1. Open a MATLAB file (`.m`) in Zed
2. Verify syntax highlighting works
3. Test hover information and completion
4. Check Extension Logs for any errors

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy --no-deps
```

## Tested Configurations

- **MATLAB**: R2025a
- **Node.js**: v22.x (Homebrew on macOS)
- **Zed**: 0.201+
- **Platforms**: macOS (primary), Linux

## Optional: Wrapper Script Approach

For simplified configuration, create a wrapper script:

1. Create `/usr/local/bin/matlabls`:
   ```bash
   #!/bin/bash
   export PATH="/Applications/MATLAB_R2025a.app/bin:$PATH"
   exec /opt/homebrew/opt/node@22/bin/node \
     /Users/username/MATLAB-language-server/out/index.js \
     --stdio \
     --matlabInstallPath /Applications/MATLAB_R2025a.app \
     "$@"
   ```

2. Make it executable:
   ```bash
   chmod +x /usr/local/bin/matlabls
   ```

3. Simplify Zed configuration:
   ```json
   {
     "lsp": {
       "matlab-language-server": {
         "binary": {
           "path": "matlabls"
         }
       }
     }
   }
   ```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project follows the same license as the underlying components it integrates with.

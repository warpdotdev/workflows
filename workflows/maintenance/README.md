# System Maintenance Workflow for macOS

A comprehensive system maintenance workflow for Warp terminal that automates package updates, system cleanup, and health monitoring on macOS systems.

## 🚀 Features

- **Automated Package Management**: Updates Homebrew, npm global packages, and Ruby gems
- **System Cleanup**: Cleans package caches, removes unused packages, empties trash
- **Health Monitoring**: Checks system status, disk space, environment variables
- **Comprehensive Summaries**: Provides detailed AI-agent-style summaries after each operation
- **Multiple Commands**: 8 specialized commands for different maintenance tasks
- **Interactive Mode**: Safe prompting before applying system changes
- **Detailed Reporting**: Generates timestamped system reports

## 📦 What It Updates

- ✅ **Homebrew packages** (formulas and casks)
- ✅ **npm global packages**
- ✅ **Ruby gems** (including RubyGems system)
- ⚠️ **Python packages** (recommends pipx/virtual environments)
- 📋 **System caches** (Homebrew, npm, Ruby)
- 🗑️ **System trash** (requires `trash` utility)

## 🛠 Installation & Setup

### Prerequisites
- macOS (tested on macOS 15.6+)
- [Homebrew](https://brew.sh) installed
- [Node.js & npm](https://nodejs.org) installed
- Ruby with rbenv (optional but recommended)

### Quick Setup

1. **Install the Warp workflow**: Place `system_maintenance.yaml` in `~/.config/warp/workflows/`
2. **Install optional dependencies**:
   ```bash
   brew install trash  # For trash cleanup functionality
   brew install pipx   # For Python package management
   ```

### Advanced Setup (Recommended)

For the full experience with command-line aliases:

1. **Download the shell script**: Copy the `system-maintenance` script to `~/.local/bin/`
2. **Make it executable**: `chmod +x ~/.local/bin/system-maintenance`
3. **Add to PATH**: Add `export PATH="$HOME/.local/bin:$PATH"` to your shell config
4. **Add aliases** to your shell config (`.zshrc`, `.bashrc`, etc.):
   ```bash
   alias sysmaint='system-maintenance all'      # Complete maintenance
   alias syscheck='system-maintenance check'    # Quick status check
   alias sysupdate='system-maintenance update'  # Package updates only
   alias sysclean='system-maintenance cleanup'  # System cleanup only
   ```

## 🎯 Usage

### Via Warp Workflow
1. Press `Cmd+Shift+P` in Warp
2. Type "System Maintenance"
3. Select your desired command

### Via Command Line (if shell script installed)
```bash
# Quick system check (recommended for daily use)
syscheck

# Complete maintenance cycle (recommended weekly)
sysmaint

# Update packages only
sysupdate

# Clean system caches only
sysclean

# Check environment variables
system-maintenance env

# System health diagnostics
system-maintenance health

# Apply quick fixes for common issues
system-maintenance fix

# Generate detailed system report
system-maintenance report
```

## 📋 Available Commands

| Command | Description | Use Case |
|---------|-------------|----------|
| **🔍 System Status Check** | Shows system info, disk space, network status | Daily monitoring |
| **📦 Check Package Updates** | Lists outdated packages without updating | Before planning updates |
| **🔄 Update All Packages** | Updates Homebrew, npm, Ruby gems | Weekly maintenance |
| **🧹 System Cleanup** | Cleans caches, removes unused packages | After updates or monthly |
| **🔧 Environment Variables Check** | Verifies dev environment setup | After new installs |
| **🩺 System Health Check** | Comprehensive system diagnostics | Troubleshooting |
| **🎯 Quick Fix Common Issues** | Applies common system fixes | When experiencing issues |
| **📊 Generate System Report** | Creates detailed system report | Documentation/auditing |

## 📈 Sample Output

Each command provides a comprehensive summary:

```
╔════════════════════════════════════════════════════════════════════════════════╗
║                           📋 MAINTENANCE SUMMARY                               ║
╚════════════════════════════════════════════════════════════════════════════════╝

🕐 Completed: Mon Aug 11 15:42:05 +03 2025
⚡ Command: syscheck (system check)

✅ Status Checks:
   • macOS 15.6 system info collected
   • Disk space verified - system healthy
   • Network connectivity verified
   • Homebrew packages are current
   • npm global packages are current
   • Ruby gems are current

💾 System Status:
   • Free disk space: 335.4 GB
   • macOS: 15.6
   • Downloads folder: 12G
   • Desktop folder: 452M
   • Cache folder: 13G

🎯 Recommendations:
   • Run 'sysupdate' to apply package updates
   • Run 'sysclean' to clean up caches
   • Consider 'sysmaint' for complete maintenance
```

## 🔐 Safety Features

- **Interactive prompts** before making system changes
- **Dry-run capabilities** for checking what will be updated
- **Comprehensive logging** of all actions taken
- **Rollback recommendations** in case of issues
- **Non-destructive operations** by default

## 📅 Recommended Usage Schedule

- **Daily**: `syscheck` (2 seconds)
- **Weekly**: `sysmaint` (5-10 minutes)
- **Monthly**: `system-maintenance report` for documentation
- **As needed**: `system-maintenance health` for troubleshooting

## 🧪 Troubleshooting

### Common Issues

**"Command not found"**
- Ensure workflow is in `~/.config/warp/workflows/`
- For shell commands, verify PATH includes `~/.local/bin`

**"Permission denied"**
- Run `chmod +x ~/.local/bin/system-maintenance`

**Updates failing**
- Run `system-maintenance health` to diagnose issues
- Check `brew doctor` output in health report

**Ruby gems not updating**
- Ensure Ruby is managed by rbenv or similar
- Check Ruby version compatibility

### Environment Variables Not Set?

The workflow checks for common development environment variables:
- `ANDROID_HOME` / `ANDROID_SDK_ROOT`
- `JAVA_HOME`
- `DOTNET_ROOT`

These warnings are informational and won't prevent the workflow from running.

## 🤝 Contributing

Found a bug or have a feature request? 

1. Test your changes thoroughly on macOS
2. Ensure all commands provide helpful summaries
3. Update documentation for any new features
4. Submit a pull request with a clear description

## 📄 License

This workflow is developed by mertkalak
This workflow is provided as-is for the Warp community. Feel free to modify and redistribute.

## 🙏 Acknowledgments

Built for the Warp terminal community. Special thanks to developers who maintain the essential tools this workflow automates: Homebrew, npm, Ruby, and the macOS ecosystem.

---

**💡 Pro Tip**: Add `sysmaint` to your weekly routine - it's like having a personal system administrator that never sleeps!

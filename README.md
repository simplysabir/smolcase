# 🔐 smolcase

**Zero-infrastructure secret management that actually works**

Stop wrestling with complex secret management tools. Smolcase stores encrypted secrets in Git repositories with zero servers, zero config files, and zero headaches.

## ⚡ Quick Start (30 seconds)

```bash
# Install
cargo install smolcase

# Create project
mkdir my-secrets && cd my-secrets
smolcase init --name "My Project" --git

# Configure once (no more password prompts!)
smolcase configure

# Add secrets
smolcase add DATABASE_URL "postgresql://..."
smolcase add API_KEY "sk-live-..." --users "alice,bob"

# Push to GitHub (fully encrypted)
git remote add origin https://github.com/you/my-secrets.git
git push -u origin main
```

**That's it.** Your secrets are encrypted, stored in Git, and accessible to your team.

## 🚀 Why you should consider?

- **🔒 Zero Knowledge**: Even with repo access, secrets stay encrypted
- **⚡ Zero Setup**: One command to configure, then seamless access
- **🌐 Git Native**: Works with GitHub, GitLab, or any Git hosting
- **👥 Team Ready**: Granular permissions, groups, role separation
- **📁 File Support**: Encrypt `.env` files, configs, certificates
- **🚫 No Servers**: No infrastructure, no vendor lock-in, just Git

## 🎯 Real-World Workflow

### Admin Setup (Once)

```bash
# Initialize project
smolcase init --name "MyApp Secrets" --git

# Configure admin credentials (cache locally)
smolcase configure
# ✓ Admin password cached
# ✓ Master key cached  
# ✓ No more repeated prompts

# Add team members
smolcase user add alice --email "alice@company.com"
smolcase user add bob --email "bob@company.com"

# Create groups
smolcase group create developers
smolcase group add-user developers alice bob

# Add secrets with permissions
smolcase add DATABASE_URL "postgresql://..." --groups "developers"
smolcase add STRIPE_KEY "sk_live_..." --users "alice"
smolcase add .env.production --groups "developers"

# Push to GitHub (encrypted)
smolcase sync
git push
```

### Team Member Setup (Once)

```bash
# Clone secrets repo
git clone https://github.com/company/myapp-secrets.git
cd myapp-secrets

# Configure user credentials (cache locally)
smolcase configure
# Username: alice
# Password: [from admin]
# Master key: [from admin]

# Access secrets (no prompts!)
smolcase get DATABASE_URL
smolcase list
smolcase export --format env > .env
```

### Daily Usage (Zero Friction)

```bash
# Get secrets instantly
smolcase get API_KEY

# Export for development
eval $(smolcase export --format env)

# Check what's available
smolcase list

# Pull latest changes
git pull && smolcase status
```

## 🛡️ Security by Design

### Military-Grade Encryption
- **ChaCha20-Poly1305**: Authenticated encryption
- **Argon2id**: Memory-hard key derivation
- **Zero Knowledge**: Admin can't see your secrets without master key

### Smart Permission System
```bash
# User-level permissions
smolcase add SECRET "value" --users "alice,bob"

# Group-level permissions  
smolcase add API_KEY "key" --groups "developers,qa"

# Public secrets (all team members)
smolcase add PUBLIC_URL "https://app.com"
```

### Role Separation
- **Admins**: Manage users, add/remove secrets, sync to Git
- **Users**: Access permitted secrets, export, list
- **Clean boundaries**: Users can't accidentally break things

## 📋 Command Reference

### Getting Started
```bash
smolcase init [--name PROJECT] [--git]    # Initialize project
smolcase configure                        # Setup credentials (once)
smolcase logout                          # Clear cached credentials
```

### Daily Commands
```bash
smolcase get <SECRET>                    # Get secret value
smolcase list                           # Show accessible secrets
smolcase export [--format env|json]     # Export secrets
smolcase status                         # Project & credential status
```

### Admin Commands
```bash
smolcase add <KEY> <VALUE>              # Add secret
smolcase remove <KEY>                   # Remove secret
smolcase user add <NAME>                # Add team member
smolcase group create <NAME>            # Create group
smolcase sync                          # Commit to Git
```

## 🔧 DevOps Integration

### GitHub Actions
```yaml
- name: Load secrets
  env:
    SMOLCASE_USER: ${{ secrets.SMOLCASE_USER }}
    SMOLCASE_PASSWORD: ${{ secrets.SMOLCASE_PASSWORD }}
    SMOLCASE_MASTER_KEY: ${{ secrets.SMOLCASE_MASTER_KEY }}
  run: |
    git clone https://github.com/company/secrets.git
    cd secrets
    smolcase configure  # Uses env vars
    smolcase export --format env >> $GITHUB_ENV
```

### Docker Development
```dockerfile
RUN curl -L https://github.com/simplysabir/smolcase/releases/latest/download/smolcase-linux.tar.gz | tar xz
COPY secrets/ ./secrets/
RUN cd secrets && smolcase export --format env > /app/.env
```

### Local Development
```bash
# One-time setup
git clone https://github.com/company/myapp-secrets.git secrets
cd secrets && smolcase configure

# Daily usage
cd secrets && smolcase export --format env > ../.env.local
source .env.local
```

## 📦 Installation

### Option 1 : From Binary releases
```bash
curl -sSL https://github.com/simplysabir/smolcase/install.sh | bash
```

### Option 2: Cargo (Recommended)
```bash
cargo install smolcase
```

### Option 3: From Source
```bash
git clone https://github.com/simplysabir/smolcase
cd smolcase && cargo build --release
sudo mv target/release/smolcase /usr/local/bin/
```

## 🆚 vs. Other Tools

| Feature | smolcase | HashiCorp Vault | AWS Secrets | SOPS |
|---------|----------|----------------|-------------|------|
| Infrastructure | None | Complex | AWS Required | None |
| Setup Time | 30 seconds | Hours | Hours | Medium |
| Team Permissions | ✅ Built-in | ✅ Complex | ✅ AWS IAM | ❌ Manual |
| Git Integration | ✅ Native | ❌ Separate | ❌ Separate | ✅ Basic |
| Credential Caching | ✅ Seamless | ❌ Manual | ❌ Manual | ❌ None |
| Cost | Free | $$ License | $$ Usage | Free |

## 🚨 Security Best Practices

### For Admins
1. **Strong Passwords**: 12+ characters for admin & master key
2. **Regular Rotation**: Change master key quarterly
3. **Least Privilege**: Only grant necessary secret access
4. **Private Repos**: Use private Git repositories
5. **Backup Strategy**: Keep encrypted repo backups

### For Team Members
1. **Secure Sharing**: Receive credentials via secure channels
2. **Local Security**: Don't commit exported `.env` files
3. **Stay Updated**: Pull changes before starting work
4. **Report Issues**: Alert admin of suspicious activity
5. **Clean Logout**: Use `smolcase logout` on shared machines

## 🛠️ Troubleshooting

**"Access denied" errors**
```bash
smolcase status          # Check your permissions
git pull origin main     # Pull latest changes
smolcase configure       # Reconfigure if needed
```

**"Invalid password" errors**
```bash
smolcase logout          # Clear cached credentials
smolcase configure       # Reconfigure with correct password
```

**Git conflicts**
```bash
git pull origin main     # Pull first
smolcase sync           # Then sync your changes
```

## 📊 What Makes This Different

Traditional secret management is **complex**:
- Infrastructure to maintain
- Complex permission systems  
- Vendor lock-in
- Expensive licensing
- Slow team onboarding

smolcase is **simple**:
- Uses Git you already have
- One-time credential setup
- Zero infrastructure costs
- Instant team access
- Open source freedom

## 🤝 Contributing

We love contributions! Here's how:

1. **Fork** the repository
2. **Create** feature branch: `git checkout -b feature/amazing-feature`
3. **Test** your changes: `cargo test`
4. **Commit** changes: `git commit -m 'Add amazing feature'`
5. **Push** to branch: `git push origin feature/amazing-feature`
6. **Open** Pull Request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Ready to simplify your secret management?**

```bash
cargo install smolcase
mkdir my-secrets && cd my-secrets
smolcase init --git
```

⭐ **Star us on GitHub** if smolcase saves you time!
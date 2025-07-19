# 🔐 smolcase

**Zero-infrastructure secret management that actually works**

Stop wrestling with complex secret management tools. Smolcase stores encrypted secrets in Git repositories with zero servers, zero config files, and zero headaches.

## ⚡ Quick Start (30 seconds)

```bash
# Install
cargo install smolcase

# Create project (interactive setup!)
mkdir my-secrets && cd my-secrets
smolcase init

# Or run the guided tutorial
smolcase tutorial

# Configure once (no more password prompts!)
smolcase configure

# Add secrets
smolcase add DATABASE_URL "postgresql://..."
smolcase add API_KEY "sk-live-..." --users "alice,bob"

# Run commands with secrets
smolcase run -- npm start
smolcase run -- docker-compose up

# Push to GitHub (fully encrypted)
git push -u origin main
```

**That's it.** Your secrets are encrypted, stored in Git, and accessible to your team.

## 🚀 Why you should consider?

- **🔒 Zero Knowledge**: Even with repo access, secrets stay encrypted
- **⚡ Zero Setup**: Interactive setup gets you started in seconds
- **🌐 Git Native**: Works with GitHub, GitLab, or any Git hosting
- **👥 Team Ready**: Granular permissions, groups, role separation
- **📁 File Support**: Encrypt `.env` files, configs, certificates
- **🚫 No Servers**: No infrastructure, no vendor lock-in, just Git
- **🎯 Developer Friendly**: Run commands directly with secrets injected

## 🎯 Real-World Workflow

### Interactive Setup (New!)

```bash
# Brand new? Start here!
smolcase tutorial
# Walks you through everything step-by-step

# Or interactive project setup
smolcase init
# → Project name: [My Secrets]
# → Initialize git repo? [Y/n] 
# → Add GitHub remote? [Y/n]
# → Repository URL: https://github.com/user/secrets
# → Admin username: admin
# → Generate master key? [Y/n]
# → Add first secret? [Y/n]
# ✓ Setup complete!
```

### Admin Setup (Once)

```bash
# Interactive initialization
smolcase init

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
```

### Daily Usage (Zero Friction)

```bash
# Run commands with secrets (NEW!)
smolcase run -- npm start
smolcase run -- python app.py
smolcase run -- docker-compose up
smolcase run --env production -- ./deploy.sh

# Apply config templates (NEW!)
smolcase apply docker-compose.template.yml > docker-compose.yml
smolcase apply nginx.conf.template --output /etc/nginx/nginx.conf

# Traditional export (still works)
smolcase export --format env > .env
eval $(smolcase export --format env)

# Get secrets instantly
smolcase get API_KEY

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
smolcase init [--name PROJECT] [--git]    # Interactive project setup
smolcase tutorial                         # Guided walkthrough (NEW!)
smolcase configure                        # Setup credentials (once)
smolcase logout                          # Clear cached credentials
```

### Daily Commands
```bash
smolcase run -- <command>               # Run command with secrets (NEW!)
smolcase apply <template> [--output]    # Process config templates (NEW!)
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

### Template Processing (NEW!)

**docker-compose.template.yml**:
```yaml
services:
  app:
    environment:
      - DATABASE_URL={{DATABASE_URL}}
      - API_KEY={{API_KEY}}
      - REDIS_URL={{REDIS_URL}}
```

**Usage**:
```bash
# Process template with secrets
smolcase apply docker-compose.template.yml > docker-compose.yml
docker-compose up

# Or run directly
smolcase run -- docker-compose -f docker-compose.template.yml up
```

**nginx.conf.template**:
```nginx
server {
    server_name {{DOMAIN_NAME}};
    ssl_certificate {{SSL_CERT_PATH}};
    ssl_certificate_key {{SSL_KEY_PATH}};
}
```

### GitHub Actions
```yaml
- name: Deploy with secrets
  env:
    SMOLCASE_USER: ${{ secrets.SMOLCASE_USER }}
    SMOLCASE_PASSWORD: ${{ secrets.SMOLCASE_PASSWORD }}
    SMOLCASE_MASTER_KEY: ${{ secrets.SMOLCASE_MASTER_KEY }}
  run: |
    git clone https://github.com/company/secrets.git
    cd secrets
    smolcase configure  # Uses env vars
    smolcase run -- ./deploy.sh
```

### Docker Development
```dockerfile
RUN curl -L https://github.com/simplysabir/smolcase/releases/latest/download/smolcase-linux.tar.gz | tar xz
COPY secrets/ ./secrets/
RUN cd secrets && smolcase apply app.template.env > /app/.env
```

### Local Development
```bash
# One-time setup
git clone https://github.com/company/myapp-secrets.git secrets
cd secrets && smolcase configure

# Daily usage - run directly
cd secrets && smolcase run -- npm start

# Or export traditionally  
cd secrets && smolcase export --format env > ../.env.local
```

## 📦 Installation

### Option 1: Cargo (Recommended)
```bash
cargo install smolcase
```

### Option 2: From Binary releases
```bash
curl -sSL https://github.com/simplysabir/smolcase/install.sh | bash
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
| Interactive Setup | ✅ Tutorial | ❌ Manual | ❌ Manual | ❌ Manual |
| Command Execution | ✅ Built-in | ❌ Manual | ❌ Manual | ❌ None |
| Template Processing | ✅ Built-in | ❌ Separate | ❌ Separate | ❌ Manual |
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

**Template not working**
```bash
# Check template syntax: {{SECRET_NAME}}
smolcase list           # Verify secret exists
smolcase get SECRET_NAME # Test secret access
```

## 📊 What Makes This Different

Traditional secret management is **complex**:
- Infrastructure to maintain
- Complex permission systems  
- Vendor lock-in
- Expensive licensing
- Slow team onboarding
- Manual export/import workflows

smolcase is **simple**:
- Uses Git you already have
- Interactive setup wizard
- One-time credential setup
- Zero infrastructure costs
- Instant team access
- Direct command execution
- Template processing built-in
- Open source freedom

## 🎓 Learning Resources

### For New Users
```bash
# Start here - complete guided tutorial
smolcase tutorial

# Quick interactive setup
smolcase init
```

### For Teams
1. **Admin sets up**: `smolcase init` → add users → push to Git
2. **Team members**: `git clone` → `smolcase configure`
3. **Daily usage**: `smolcase run -- <command>`

### Example Workflows

**Frontend Development**:
```bash
# Setup once
smolcase add API_URL "https://api.staging.com"
smolcase add API_KEY "dev-key-123"

# Daily development
smolcase run -- npm start
smolcase run -- yarn build
```

**Backend Deployment**:
```bash
# Template-based deployment
smolcase apply k8s-deployment.template.yaml > deployment.yaml
kubectl apply -f deployment.yaml

# Or direct execution
smolcase run -- kubectl apply -f k8s-deployment.template.yaml
```

**Docker Compose**:
```bash
# Process template
smolcase apply docker-compose.template.yml > docker-compose.yml
docker-compose up

# Or run directly (if Docker supports templates)
smolcase run -- docker-compose up
```

## 🤝 Contributing

We love contributions! Here's how:

1. **Fork** the repository
2. **Create** feature branch: `git checkout -b feature/amazing-feature`
3. **Test** your changes: `cargo test`
4. **Commit** changes: `git commit -m 'Add amazing feature'`
5. **Push** to branch: `git push origin feature/amazing-feature`
6. **Open** Pull Request

### Development Setup
```bash
git clone https://github.com/simplysabir/smolcase
cd smolcase
cargo build
cargo test

# Test locally
./target/debug/smolcase tutorial
```

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

**Ready to simplify your secret management?**

```bash
cargo install smolcase
mkdir my-secrets && cd my-secrets
smolcase init  # Interactive setup!
# or
smolcase tutorial  # Full guided walkthrough
```

⭐ **Star us on GitHub** if smolcase saves you time!

---

## 🆕 What's New in v1.3.0

- **🎯 Interactive Setup**: `smolcase init` now guides you through setup
- **📚 Built-in Tutorial**: `smolcase tutorial` for complete walkthrough  
- **🚀 Command Execution**: `smolcase run -- <command>` runs commands with secrets
- **📝 Template Processing**: `smolcase apply template.yml` processes config templates
- **🎨 Better UX**: Improved prompts, guidance, and error messages
- **⚡ Environment Filtering**: `--env production` filters secrets by environment
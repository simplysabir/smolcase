# 🔐 smolcase

**Secure team secret management CLI tool built in Rust**

Zero-infrastructure secret sharing for development teams. Store encrypted secrets in Git repositories without revealing any sensitive data - even if the repo is public!

## 🚀 Why smolcase?

- **🔒 Zero Knowledge**: Even with repo access, secrets remain encrypted
- **🌐 Git-Native**: Works with GitHub, GitLab, or any Git hosting
- **👥 Team-Friendly**: Granular permissions per user/group
- **📁 File Support**: Encrypt entire files (like `.env`, configs)
- **🚫 No Servers**: No infrastructure needed, just Git
- **⚡ Simple**: One binary, clear workflow

## 📦 Installation

### From Source
```bash
git clone https://github.com/simplysabir/smolcase
cd smolcase
cargo build --release
sudo cp target/release/smolcase /usr/local/bin/
```

### Using Cargo
```bash
cargo install smolcase
```

## 🎯 Real-World Workflow

### As a Team Manager/Admin

**1. Initialize Your Project**
```bash
mkdir myproject-secrets
cd myproject-secrets
smolcase init --name "MyProject Secrets" --git

# You'll be prompted for:
# - Admin username: john_admin
# - Admin email: john@company.com  
# - Admin password: [secure password for admin operations]
# - Master decryption key: [shared key for team - like "MyProject2024!"]
```

**2. Add Team Members**
```bash
# Add developers
smolcase user add alice --email "alice@company.com"
smolcase user add bob --email "bob@company.com"
smolcase user add charlie --email "charlie@company.com"

# Add other roles
smolcase user add sarah_qa --email "sarah@company.com"
smolcase user add mike_devops --email "mike@company.com"
```

**3. Create Groups for Better Management**
```bash
# Create groups
smolcase group create developers --description "Backend developers"
smolcase group create frontend --description "Frontend team"
smolcase group create qa --description "QA team"
smolcase group create devops --description "DevOps engineers"

# Add users to groups
smolcase group add-user developers alice bob
smolcase group add-user frontend charlie
smolcase group add-user qa sarah_qa
smolcase group add-user devops mike_devops
```

**4. Add Secrets with Granular Permissions**
```bash
# Database credentials - only for backend devs
smolcase add DATABASE_URL "postgresql://prod:password@db.company.com:5432/myapp" --groups "developers,devops"

# API keys - different access levels
smolcase add STRIPE_SECRET_KEY "sk_live_..." --groups "developers"
smolcase add STRIPE_PUBLIC_KEY "pk_live_..." --groups "developers,frontend"

# Third-party integrations
smolcase add SENDGRID_API_KEY "SG...." --users "alice,mike_devops"
smolcase add GOOGLE_ANALYTICS_ID "GA-..." --groups "frontend,qa"

# Environment files
smolcase add .env.production --groups "developers,devops"
smolcase add .env.staging --groups "developers,frontend,qa"

# Admin-only secrets
smolcase add ROOT_SSH_KEY "-----BEGIN PRIVATE KEY-----..." --users "mike_devops"
```

**5. Push to GitHub/GitLab**
```bash
# Add remote repository
git remote add origin https://github.com/company/myproject-secrets.git

# Push encrypted configuration
git push -u origin main

# The repository now contains:
# - .smolcase.yml (encrypted configuration)
# - README.md (optional: document your secret structure)
```

**6. Share Credentials with Team**

Send each team member **securely** (via encrypted email, Slack DM, etc.):
```
Hi Alice,

Access details for MyProject secrets:
- Repository: https://github.com/company/myproject-secrets
- Your username: alice
- Your password: xMLpDTVGCwjwYrfU
- Master decryption key: MyProject2024!

Setup instructions: https://github.com/company/myproject-secrets#setup
```

### As a Team Member

**1. Clone and Setup**
```bash
# Clone the secrets repository
git clone https://github.com/company/myproject-secrets.git
cd myproject-secrets

# Verify you have access
smolcase setup
# Enter your username and password when prompted
```

**2. Access Your Secrets**
```bash
# List secrets you have access to
smolcase list

# Get specific secrets
smolcase get DATABASE_URL
# Enter your password, then master decryption key

# Export to environment file
smolcase export --format env > .env
# Enter your password, then master decryption key

# Export to JSON for scripts
smolcase export --format json > secrets.json
```

**3. Use in Development**
```bash
# Load secrets into your shell
eval $(smolcase export --format env)

# Or save to file and source
smolcase export --format env > .env.local
source .env.local

# Your app can now use: $DATABASE_URL, $STRIPE_SECRET_KEY, etc.
```

**4. Stay Updated**
```bash
# Pull latest secret changes
git pull origin main

# Check what's new
smolcase status
smolcase list
```

## 🛠️ Command Reference

### Project Management
```bash
smolcase init [--name PROJECT] [--git]    # Initialize new project
smolcase status                           # Show project status  
smolcase sync                            # Commit changes to git
```

### Secret Management
```bash
smolcase add <KEY> [VALUE]               # Add secret (prompts if no value)
smolcase add <FILE_PATH>                 # Encrypt entire file
smolcase get <KEY>                       # Retrieve secret value
smolcase remove <KEY>                    # Delete secret
smolcase list                           # List accessible secrets
```

### Permissions
```bash
# Add with specific permissions
smolcase add API_KEY "secret" --users "alice,bob"
smolcase add DATABASE_URL "url" --groups "developers,devops"

# No permissions = accessible by all users
smolcase add PUBLIC_KEY "key"
```

### User Management (Admin Only)
```bash
smolcase user add <USERNAME> [--email EMAIL]    # Add user
smolcase user list                              # List all users
smolcase user reset <USERNAME>                  # Reset password
smolcase user remove <USERNAME>                 # Remove user
```

### Group Management (Admin Only)
```bash
smolcase group create <NAME> [--description DESC]      # Create group
smolcase group list                                    # List groups
smolcase group add-user <GROUP> <USER1> [USER2...]    # Add users to group
smolcase group remove-user <GROUP> <USER1> [USER2...] # Remove users
smolcase group delete <NAME>                          # Delete group
```

### Import/Export
```bash
smolcase export [--format env|json|yaml] [--output FILE]    # Export secrets
smolcase import <FILE> [--format env|json|yaml]             # Import secrets
```

### Repository Access
```bash
smolcase setup [--repo URL]             # Setup access to repo
```

## 🔧 Integration Examples

### CI/CD Pipeline (GitHub Actions)
```yaml
# .github/workflows/deploy.yml
name: Deploy
on: [push]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      # Checkout secrets repo
      - name: Get secrets
        run: |
          git clone https://github.com/company/myproject-secrets.git secrets
          cd secrets
          
      # Install smolcase (in real scenario, cache this)
      - name: Install smolcase
        run: |
          curl -L https://github.com/yourusername/smolcase/releases/latest/download/smolcase-linux.tar.gz | tar xz
          sudo mv smolcase /usr/local/bin/
          
      # Export secrets
      - name: Load secrets
        env:
          SMOLCASE_USER: ${{ secrets.SMOLCASE_CI_USER }}
          SMOLCASE_PASSWORD: ${{ secrets.SMOLCASE_CI_PASSWORD }}
          SMOLCASE_MASTER_KEY: ${{ secrets.SMOLCASE_MASTER_KEY }}
        run: |
          cd secrets
          echo "$SMOLCASE_PASSWORD" | smolcase get DATABASE_URL | echo "DATABASE_URL=$(cat)" >> $GITHUB_ENV
          echo "$SMOLCASE_PASSWORD" | smolcase get API_KEY | echo "API_KEY=$(cat)" >> $GITHUB_ENV
          
      - name: Deploy
        run: |
          echo "Deploying with DATABASE_URL=$DATABASE_URL"
          # Your deployment script here
```

### Docker Development
```dockerfile
# Dockerfile.dev
FROM node:18
WORKDIR /app

# Install smolcase
RUN curl -L https://github.com/yourusername/smolcase/releases/latest/download/smolcase-linux.tar.gz | tar xz && \
    mv smolcase /usr/local/bin/

# Copy secrets repo (in build context)
COPY secrets/ ./secrets/

# Export secrets on container start
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
```

```bash
#!/bin/bash
# entrypoint.sh
cd /app/secrets
smolcase export --format env > /app/.env
cd /app
exec "$@"
```

### Development Scripts
```bash
#!/bin/bash
# scripts/dev-setup.sh

echo "Setting up development environment..."

# Clone secrets if not exists
if [ ! -d "secrets" ]; then
    git clone https://github.com/company/myproject-secrets.git secrets
fi

cd secrets
git pull origin main

echo "Exporting development secrets..."
smolcase export --format env > ../.env.local

echo "✅ Environment ready! Run: source .env.local"
```

## 🔒 Security Features

### Encryption
- **Algorithm**: ChaCha20-Poly1305 (authenticated encryption)
- **Key Derivation**: Argon2id (memory-hard, quantum-resistant)
- **Master Key**: Separate from admin credentials
- **Per-Secret Permissions**: Zero-knowledge access control

### Access Control
- **User Authentication**: Individual password per team member
- **Group Permissions**: Organize users into logical groups
- **Secret-Level Permissions**: Each secret can have specific user/group access
- **Admin Separation**: Admin operations require separate password

### Git Safety
- **Public Repo Safe**: All sensitive data encrypted before storage
- **Metadata Protected**: No usernames, secret names, or structure revealed
- **History Safe**: Past commits don't leak information
- **Audit Trail**: Git history shows when secrets were modified

## 🧪 Testing

### Run Test Suite
```bash
cargo test
```

## 🚨 Best Practices

### For Admins
1. **Use Strong Passwords**: Both admin and master key should be 12+ characters
2. **Rotate Master Key**: Change master key quarterly, re-encrypt all secrets
3. **Principle of Least Privilege**: Only give access to secrets users actually need
4. **Regular Audits**: Review user list and permissions monthly
5. **Backup Strategy**: Keep encrypted backups of the Git repository

### For Team Members
1. **Secure Storage**: Don't store passwords in plain text
2. **Environment Isolation**: Use different secrets for dev/staging/prod
3. **Local Security**: Don't commit exported `.env` files
4. **Access Updates**: Pull latest secrets before starting work
5. **Report Issues**: Tell admin immediately if you suspect compromise

### For Organizations
1. **Private Repositories**: Use private Git repos for sensitive projects
2. **Access Reviews**: Regular review of who has repository access
3. **Incident Response**: Have a plan for key rotation if compromised
4. **Documentation**: Keep track of which secrets are used where
5. **Compliance**: Ensure this meets your security/compliance requirements

## 🆘 Troubleshooting

### Common Issues

**"Access denied" when you should have permission**
```bash
# Check your username and permissions
smolcase status
smolcase user list  # (admin only)

# Verify you're in the right repository
git remote -v
```

**"Invalid master key" error**
```bash
# Verify with admin that master key hasn't changed
# Check for typos - master key is case-sensitive
```

**"Secret not found"**
```bash
# List available secrets
smolcase list

# Check if you pulled latest changes
git pull origin main
```

**Git conflicts on .smolcase.yml**
```bash
# Pull first, then sync
git pull origin main
smolcase sync
```

### Getting Help
1. Check the command help: `smolcase <command> --help`
2. Review the status: `smolcase status`
3. Check Git status: `git status`
4. Contact your admin for access issues
5. File issues at: https://github.com/simplysabir/smolcase/issues

## 📋 Migration Guide

### From Environment Files
```bash
# Import existing .env file
smolcase import .env.production --format env

# Set permissions after import
smolcase remove OLD_SECRET
smolcase add OLD_SECRET "value" --groups "developers"
```

### From Other Secret Managers
```bash
# Export from other tools to JSON
# Then import
smolcase import secrets.json --format json
```

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Make changes and add tests
4. Run test suite: `cargo test`
5. Commit changes: `git commit -m 'Add amazing feature'`
6. Push to branch: `git push origin feature/amazing-feature`
7. Open Pull Request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---
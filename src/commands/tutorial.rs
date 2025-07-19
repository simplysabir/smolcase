use crate::commands::{add, export, init, run};
use crate::ui::UI;
use anyhow::Result;
use colored::*;
use std::io::{self, Write};

pub async fn execute() -> Result<()> {
    UI::print_banner();

    println!(
        "\n{}",
        "🎯 Welcome to the smolcase Tutorial!".bold().green()
    );
    println!(
        "{}",
        "This interactive tutorial will walk you through setting up and using smolcase.".dimmed()
    );
    println!();

    // Step 1: Introduction
    tutorial_step(1, "Understanding smolcase", || {
        println!("smolcase is a zero-infrastructure secret management tool that:");
        println!("  🔐 Encrypts secrets with military-grade encryption");
        println!("  📁 Stores everything in Git repositories");
        println!("  👥 Supports team collaboration with permissions");
        println!("  ⚡ Requires zero servers or complex setup");
        println!();
        println!("Let's get started!");
    })?;

    // Step 2: Project initialization
    tutorial_step(2, "Initialize your first project", || {
        println!("We'll create a new smolcase project.");
        println!("This creates:");
        println!("  • .smolcase.yml (encrypted configuration)");
        println!("  • .smolcase/ directory (local cache)");
        println!("  • Git repository (optional)");
    })?;

    // Check if already in a project
    if crate::config::ConfigManager::is_smolcase_project() {
        UI::info("You're already in a smolcase project!");
        if !UI::confirm("Continue with tutorial using this project?")? {
            return Ok(());
        }
    } else {
        println!("\n{}", "Let's initialize a new project...".cyan());

        // Run init command
        init::execute(Some("Tutorial Project".to_string()), true, false).await?;

        UI::success("Project initialized! 🎉");
    }

    // Step 3: Adding secrets
    tutorial_step(3, "Adding your first secret", || {
        println!("Now let's add a secret. Secrets can be:");
        println!("  • Environment variables (DATABASE_URL, API_KEY)");
        println!("  • Configuration values");
        println!("  • Files (.env, certificates, configs)");
        println!();
        println!("We'll add a sample API key...");
    })?;

    println!("\n{}", "Adding a sample secret...".cyan());
    add::execute(
        "TUTORIAL_API_KEY".to_string(),
        Some("sk-tutorial-1234567890abcdef".to_string()),
        None,
        None,
    )
    .await?;

    // Step 4: Viewing secrets
    tutorial_step(4, "Viewing your secrets", || {
        println!("You can view your secrets with:");
        println!("  • smolcase list    - Show all accessible secrets");
        println!("  • smolcase get KEY - Get a specific secret value");
        println!("  • smolcase status  - Show project overview");
    })?;

    println!("\n{}", "Let's see what secrets we have...".cyan());
    crate::commands::list::execute().await?;

    // Step 5: Exporting secrets
    tutorial_step(5, "Exporting secrets for development", || {
        println!("For development, you often need secrets as environment variables:");
        println!("  • smolcase export                    - Print as KEY=value");
        println!("  • smolcase export --format json      - Export as JSON");
        println!("  • smolcase export --output .env      - Save to file");
        println!();
        println!("Let's export our secrets:");
    })?;

    println!("\n{}", "Exporting secrets...".cyan());
    export::execute("env".to_string(), None, None).await?;

    // Step 6: Running commands with secrets
    tutorial_step(6, "Running commands with secrets", || {
        println!("Instead of manually exporting, you can run commands directly:");
        println!("  • smolcase run -- node app.js");
        println!("  • smolcase run -- python main.py");
        println!("  • smolcase run -- docker-compose up");
        println!();
        println!("Let's try running a simple command:");
    })?;

    println!(
        "\n{}",
        "Running 'env | grep TUTORIAL' with secrets...".cyan()
    );
    run::execute(None, vec!["env".to_string()])
        .await
        .unwrap_or_else(|_| {
            // Command might fail on some systems, that's ok for tutorial
            UI::info("Command executed (output may vary by system)");
        });

    // Step 7: Team collaboration
    tutorial_step(7, "Team collaboration", || {
        println!("To collaborate with your team:");
        println!();
        println!("1. Push to Git repository:");
        println!("   git push origin main");
        println!();
        println!("2. Team members clone and configure:");
        println!("   git clone <repo-url>");
        println!("   smolcase configure");
        println!();
        println!("3. Add team members (as admin):");
        println!("   smolcase user add alice");
        println!("   smolcase group create developers");
        println!();
        println!("4. Set secret permissions:");
        println!("   smolcase add SECRET value --users alice,bob");
        println!("   smolcase add API_KEY key --groups developers");
    })?;

    // Step 8: Advanced features
    tutorial_step(8, "Advanced features", || {
        println!("More powerful features:");
        println!();
        println!("• Template processing:");
        println!("  smolcase apply config.template.yml > config.yml");
        println!();
        println!("• Different environments:");
        println!("  smolcase run --env production -- ./deploy.sh");
        println!();
        println!("• File encryption:");
        println!("  smolcase add .env.production");
        println!("  smolcase add ssl-cert.pem --users admin");
        println!();
        println!("• Git integration:");
        println!("  smolcase sync  # Commit changes");
    })?;

    // Step 9: Security best practices
    tutorial_step(9, "Security best practices", || {
        println!("Keep your secrets secure:");
        println!();
        println!("🔐 Use strong passwords (12+ characters)");
        println!("🔄 Rotate master key regularly");
        println!("👥 Use principle of least privilege");
        println!("🚫 Never commit .env files to Git");
        println!("🔒 Use private Git repositories");
        println!("📱 Use 'smolcase logout' on shared machines");
        println!("⚠️  Share credentials via secure channels only");
    })?;

    // Final step
    println!("\n{}", "🎉 Tutorial Complete!".bold().green());
    println!();
    println!("You now know how to:");
    println!("  ✓ Initialize smolcase projects");
    println!("  ✓ Add and manage secrets");
    println!("  ✓ Export secrets for development");
    println!("  ✓ Run commands with secrets");
    println!("  ✓ Collaborate with your team");
    println!();

    println!("{}", "Next steps:".bold().cyan());
    println!("  • Run 'smolcase --help' to see all commands");
    println!("  • Check 'smolcase status' for project overview");
    println!("  • Visit https://github.com/simplysabir/smolcase for docs");
    println!();

    if UI::confirm("Would you like to see the project status?")? {
        println!();
        crate::commands::status::execute().await?;
    }

    UI::success("Happy secret managing! 🔐");

    Ok(())
}

fn tutorial_step<F>(step: u8, title: &str, content: F) -> io::Result<()>
where
    F: FnOnce(),
{
    println!("\n{}", format!("Step {}: {}", step, title).bold().cyan());
    println!("{}", "─".repeat(50).cyan());

    content();

    print!("\n{} ", "Press Enter to continue...".dimmed());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}

use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[command(author = "Alberson da Silva Miranda", version, about)]

struct Cli {
  packages: Vec<String>,
  
  #[arg(short, long, default_value = "https://cloud.r-project.org/")]
  repository: String,
  
  #[arg(short, long)]
  library: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Cli::parse();
  let pkgs = args.packages;
  let repos = args.repository;
  let lib = args.library;
  
  let pkg_list = pkgs
      .iter()
      .map(|p| format!("\"{}\"", p))
      .collect::<Vec<_>>()
      .join(", ");
  
  let mut expr = format!(
      "install.packages(c({}), repos = \"{}\"",
      pkg_list,
      repos
  );
  if let Some(lib) = lib {
      expr.push_str(&format!(", lib = \"{}\"", lib));
  }
  expr.push(')');
  
  let status = Command::new("Rscript")
      .arg("-e")
      .arg(expr)
      .status()?;
  if !status.success() {
      std::process::exit(status.code().unwrap_or(1));
  }
  Ok(())
}

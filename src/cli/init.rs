use std::path::Path;
use crate::internal::project::Project;

pub async fn init_project() -> anyhow::Result<()> {
    let manifest = Path::new("luapm.json");
    let root     = std::env::current_dir()?;

    if manifest.exists() {
        eprintln!("luapm.json already exists in this directory");
        return Ok(())
    }
    let dir_name = std::env::current_dir()?
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "luapm-project".to_string());

    let project = Project::new(dir_name, root);
    project.save().await?;
    println!("Inited project");
    
    Ok(())
}
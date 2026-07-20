use std::env;
use std::path::PathBuf;
use crate::internal::environment::Environment;
use crate::internal::project::{project_created, Project};

pub async fn run_project(target: Option<String>) -> anyhow::Result<()> {
    let env = Environment::new()?;
    match target {
        Some(path) => {
            let path = PathBuf::from(path);
            env.run_file(&path).await?;
        },
        None => {
            let pwd = env::current_dir()?;
            let project: Project;
            if project_created(&pwd)? {
                project = Project::load(&pwd).await?;
            } else {
                let name = pwd
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "project".to_string());
                project = Project::new(name, pwd.clone()).await?;
                println!("[LuaPM] Auto inited project's luapm.json.");
            }

            let mut entry_name = project.entry.unwrap_or_else(|| "".to_string());
            let mut entry_path = None;
            if !entry_name.is_empty() && !entry_name.contains(".lua") {
                entry_name = format!("{}.lua", entry_name);
            }
            if !entry_name.is_empty() {
                entry_path = Some(project.root.join(entry_name));
            }

            env.run_many(&pwd, &entry_path, &project.contents).await?
        }
    };

    Ok(())
}
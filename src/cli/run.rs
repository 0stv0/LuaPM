use std::path::PathBuf;
use crate::internal::environment::Environment;
use crate::internal::project::Project;

pub async fn run_project(target: Option<String>) -> anyhow::Result<()> {
    let env = Environment::new()?;
    match target {
        Some(path) => {
            let path = PathBuf::from(path);
            env.run_file(&path).await?;
        },
        None => {
            let project = Project::load_default().await?;

        }
    };

    Ok(())
}
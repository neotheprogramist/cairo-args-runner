use anyhow::Result;

use crate::utils::compile::ProjectCompiler;

pub trait ScarbProjectGenerator<T> {
    fn generate(self) -> Result<T>;
}

pub struct Generator {
    folder: String,
    package: String,
}

impl Generator {
    pub fn new(folder: impl Into<String>, package: impl Into<String>) -> Self {
        Generator {
            folder: folder.into(),
            package: package.into(),
        }
    }
}

impl ScarbProjectGenerator<ProjectCompiler> for Generator {
    fn generate(self) -> Result<ProjectCompiler> {
        Ok(ProjectCompiler::new(self.folder, self.package))
    }
}

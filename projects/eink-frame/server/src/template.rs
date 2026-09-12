use anyhow::{Context, Result};
use minijinja::{Environment, path_loader};
use std::path::PathBuf;

use crate::model::Dashboard;

pub struct Templates {
    env: Environment<'static>,
}

impl Templates {
    pub fn load() -> Result<Self> {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
        let mut env = Environment::new();
        env.set_loader(path_loader(dir));
        Ok(Self { env })
    }

    pub fn render_dashboard(&self, dash: &Dashboard) -> Result<String> {
        let tmpl = self
            .env
            .get_template("dashboard.html")
            .context("templates/dashboard.html")?;
        Ok(tmpl.render(dash)?)
    }

    pub fn render_preview(&self, dash: &Dashboard) -> Result<String> {
        let tmpl = self
            .env
            .get_template("preview.html")
            .context("templates/preview.html")?;
        Ok(tmpl.render(dash)?)
    }
}

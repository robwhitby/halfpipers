use crate::{Env, Issues, Lint, Manifest, Render};

pub struct App<L, R>
where
    L: Lint,
    R: Render,
{
    env: Env,
    linter: L,
    renderer: R,
}

impl<L, R> App<L, R>
where
    L: Lint,
    R: Render,
{
    pub fn new(env: Env, linter: L, renderer: R) -> Self {
        Self { env, linter, renderer }
    }

    pub fn lint(&self, manifest: &Manifest) -> Issues {
        self.linter.lint(&self.env, manifest)
    }

    pub fn render(&self, manifest: &Manifest) -> String {
        self.renderer.render(manifest)
    }
}

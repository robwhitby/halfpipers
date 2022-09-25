use crate::Manifest;

pub trait Render {
    fn render(&self, manifest: &Manifest) -> String;
}

pub struct ConcourseRenderer;

impl Render for ConcourseRenderer {
    fn render(&self, manifest: &Manifest) -> String {
        format!("concourse renderer \n{}", HalfpipeRenderer.render(manifest))
    }
}

pub struct ActionsRenderer;

impl Render for ActionsRenderer {
    fn render(&self, manifest: &Manifest) -> String {
        format!("concourse renderer \n{}", HalfpipeRenderer.render(manifest))
    }
}

pub struct HalfpipeRenderer;

impl Render for HalfpipeRenderer {
    fn render(&self, manifest: &Manifest) -> String {
        manifest.to_yaml().unwrap()
    }
}

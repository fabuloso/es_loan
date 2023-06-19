pub enum Command {
    Capture(Capture),
    Release(Release),
    Fuck(Fuck),
}

pub struct Capture {
    pub name: String,
}
pub struct Release {}
pub struct Fuck {}

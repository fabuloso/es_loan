pub enum Command {
    Capture(Capture),
    Release(Release),
    Fuck(Fuck),
}

pub struct Capture {}
pub struct Release {}
pub struct Fuck {}

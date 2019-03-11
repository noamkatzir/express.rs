#[derive(Clone,Debug)]
pub enum MethodKind {
    Get,
    Post,
    Put,
    Delete,
    Unknown
}
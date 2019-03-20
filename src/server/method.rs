#[derive(Clone,Debug,PartialEq, Eq, Hash)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Delete,
    Unknown
}
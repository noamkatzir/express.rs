#[derive(Clone)]
pub enum MethodKind {
    Get,
    Post,
    Put,
    Delete
}
#[derive(Clone)]
pub enum HttpVersion {
    V1,
    V2
}
#[derive(Clone)]
pub enum Connection {
    KeepAlive,
    Close
}
#[derive(Clone)]
pub enum CacheControl {
    MaxAge(i32),
    NoCache
}
#[derive(Clone)]
pub enum HttpHeader {
    Host {name: String, port: i32 },
    UserAgent(String),
    Referer(String),
    Accept(Vec<(String,f32)>),
    AcceptLanguage(Vec<(String,f32)>),
    AcceptEncoding(Vec<String>),
    Connection(Connection),
    UpgradeInsecureRequests,
    CacheControl(CacheControl),
    Pragma(String)
}
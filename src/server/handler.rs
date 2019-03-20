pub trait RequestHandler: Send + Sync + 'static {
    fn action(&self,request: String) -> String;
}

impl<F> RequestHandler for F
where
    F: Send + Sync + 'static + Fn(String) -> String,
{
    fn action(&self,request: String) -> String {
        (*self)(request)
    }
}
impl RequestHandler for Box<RequestHandler> {
    fn action(&self,request: String) -> String {
         (**self).action(request)
    }
}
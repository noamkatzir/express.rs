use std::collections::HashMap;
use bytes::Bytes;
use crate::http::{RequestBuilder, Request, Response};
use super::reader::{RouteReader,Event};

pub type RouterResult = Result<Response,()>;

pub trait RouteAction: Send + Sync + 'static {
    fn action(&self,req: Request, res: Response) -> Response;
}

impl<F> RouteAction for F
where
    F: Send + Sync + 'static + Fn(Request,Response) -> Response,
{
    fn action(&self,req: Request, res: Response) -> Response {
        (*self)(req, res)
    }
}
impl RouteAction for Box<RouteAction> {
    fn action(&self,req: Request, res: Response) -> Response {
         (**self).action(req, res)
    }
}

struct Handler {
    key_name: Bytes,
    nested: HashMap<Bytes,Handler>,
    cb: Option<Box<RouteAction>>
}

pub struct Routes {
    root: Handler
}

impl Routes {
    pub fn new() -> Self
    {
        Routes {
            root: Handler {
                key_name: Bytes::new(),
                nested: HashMap::new(),
                cb: None
            }
        }
    }

    pub fn set_route<F: RouteAction + Sized> (&mut self, uri: &'static [u8],cb: F) {
        let reader = RouteReader::new(Bytes::from_static(uri));
        let mut node = &mut self.root;

        for event in reader {
            match event {
                Event::Segment(seg) => {
                    println!("segment => {:?}", seg);
                    node = moving(node).nested.entry(seg.clone()).or_insert(Handler { 
                        key_name: seg.clone(),
                        nested: HashMap::new(),
                        cb: None
                    });
                },
                Event::Variable(_) => { 
                    node = moving(node).nested.entry(Bytes::from_static(b":")).or_insert(Handler { 
                        key_name: Bytes::new(),
                        nested: HashMap::new(),
                        cb: None
                    });
                }
            }
        }

        node.cb = Some(Box::new(cb));
    }

    pub fn run_callback(&self, mut request: RequestBuilder, response: Response) -> RouterResult {
        let mut node = &self.root;
        let uri = request.uri();
        let mut uri_params = HashMap::new();

        for slice in uri.split(|ch| ch == &b'/') {
            node = match node.nested.get(slice) {
                Some(nested_node) => nested_node,
                _ => match node.nested.get(&Bytes::from_static(b":")) {
                        Some(nested_var_node) => {
                            uri_params.insert(nested_var_node.key_name.clone(), slice);
                            nested_var_node
                        },
                        _ => return Err(())
                    }
            }
        };

        //TODO: need to finish the uri_params copy + memory alocation, and return it
        //point to think of, maybe I should split to 2 methods 
        //because there is no single responsibility for this method
        //to create runner and actions is also an option
        let res: HashMap<_,_> = uri_params.iter().map(|(key,value)| (key.clone(), Bytes::from(*value))).collect();
        
        match node.cb {
            Some(ref handler) => {
                request.add_uri_params(res);
                Ok(handler.action(request.build(),response))
            },
            _ => Err(())
        }
        
    }
}
/* fix issue with the borrow checker in recurtion data staructure with:
    https://users.rust-lang.org/t/implementing-a-very-basic-trie/10788/3
*/
fn moving<T>(t: T) -> T { t }
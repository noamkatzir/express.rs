use bytes::Bytes;
use express::router::router::Router;
use express::http::{Request, Response};

fn main() {
    let mut router = Router::new();

    router.get(b"/", |_req: Request, mut res: Response| {
        let headers = _req.headers();
        let x = (headers.keys(), headers.values());
        println!("{:?}",x);
        res.add_json_headers();
        res.add_full_body(Bytes::from_static(b"{\"key\":\"value1\"}"));
        res
    });

    router.get(b"/home/noam2/page2", |_req: Request, mut res: Response| {
        res.add_json_headers();
        res.add_full_body(Bytes::from_static(b"{\"key\":\"value2\"}"));
        res
    });


    // router.get(b"/home/noam/page", page);

    // router.get(b"/", home);

    loop {
        router.bind("localhost", 8080);
    }
}
use bytes::Bytes;
use express::router::router::Router;
use express::http::{Request, Response};


// fn page (_req: Request, mut res: Response) -> Response {
//     res.add_json_headers();
//     res.add_full_body(Bytes::from_static(b"{\"key\":\"value\"}"));
//     res
// }

// fn home (_req: Request, mut res: Response) -> Response {
//     res.add_json_headers();
//     res.add_full_body(Bytes::from_static(b"{\"home\":\"value\"}"));
//     res
// }

fn main() {
    let mut router = Router::new();

    router.get(b"/home/noam/page", |_req: Request, mut res: Response| {
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
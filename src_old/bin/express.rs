use bytes::Bytes;
use std::io::prelude::*;
use express::router::router::Router;
use express::http::{Request, Response};

fn main() {
    let mut router = Router::new();

    router.get(b"/home", |_req: Request, mut res: Response| {
        // let headers = _req.headers();
        // let x = (headers.keys(), headers.values());
        // println!("{:?}",x);
        // res.add_html_headers();
        // res.add_full_body(Bytes::from_static(b"{\"key\":\"value1\"}"));

        // use flate2::Compression;
        // use flate2::write::ZlibEncoder;
        use std::fs::read_to_string;
        // let mut e = ZlibEncoder::new(Vec::new(), Compression::fast());
        let content = read_to_string("/home/noam/dev/workspace/express/src/bin/temp.txt").unwrap();
        // e.write_all(content.as_bytes());
        // let a = e.finish().unwrap();
        res.add_html_headers().add_gzip_headers();
        res.add_full_body(Bytes::from(content));

        res
    });

    router.get(b"/home/noam2/page2", |_req: Request, mut res: Response| {
        res.add_json_headers();
        res.add_full_body(Bytes::from_static(b"{key:\"value1\", value: {key:\"value1\", value: {key:\"value1\", value: {key:\"value1\"}}}}"));
        res
    });


    // router.get(b"/home/noam/page", page);

    // router.get(b"/", home);

    loop {
        router.bind("localhost", 8080);
    }
}
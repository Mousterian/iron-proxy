extern crate iron;
extern crate hyper;

use iron::prelude::*;
use iron::status;
use iron::{Handler};

use hyper::Client;
use hyper::header::{Connection, Headers};
use std::io::Read;

use iron::mime::Mime;
use iron::modifier::Modifier;

pub struct AllHeadersReplaceModifier<Headers>{
    pub headers: Headers
}

impl Modifier<Response> for AllHeadersReplaceModifier<Headers> {
    #[inline]
    fn modify(self, res: &mut Response) {
        res.headers = self.headers.clone();
    }
}

struct PProxy {
}

impl PProxy {
    fn new() -> Self {
        PProxy {
        }
    }

}

impl Handler for PProxy {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut str_path = String::new();
        for s in req.url.path().into_iter() {
            str_path.push('/');
            str_path.push_str(s);
        }
        let uri = "http://rust-lang.org".to_string() + &str_path;
        println!("Requesting url {}", uri);
        let client = Client::new(); //Client::with_http_proxy("",3128);
        let mut response : hyper::client::Response = client.get(&uri)
                        .header(Connection::close())
                        .send().unwrap();

        let mut response_bytes : Vec<u8> = Vec::new();
        response.read_to_end(&mut response_bytes).unwrap();

        let ref headers = response.headers;
        // weird orphan types rules mean I can't actually specialise Modifier here in the manner
        // of the iron doco, perhaps this could be boiled down to a roll-your-own modifier example
        // in iron?
        let set_wrapper = AllHeadersReplaceModifier{ headers: headers.clone() };
        let iron_response = Response::with((response.status, response_bytes));

        Ok(iron_response.set(set_wrapper))
    }
}

fn main() {
    let proxy = PProxy::new();
    Iron::new(proxy).http("localhost:3000").unwrap();
}
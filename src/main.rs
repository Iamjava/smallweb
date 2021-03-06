use smallweb::*;
use smallweb::HTTP_RESPONSE::*;
use std::thread;


fn main(){
    let router = Router::new().get("/hello",|r:Request|_OK("Hello".to_string()))
        .validator(|r:Request| Some(r))
        .default(_OK("NOT FOUND".to_string()))
        .get("/saymyname", hello_params) // responds to /saymyname&name=jan
        .get("/:name", hello_url_param)
        .get("/",|r:Request| _OK("<h1>HELLO from the BASE</h1>".to_string()))
        .thradpool_size(4)
        .okay();

    serve("127.0.0.1:7000",router);
}

fn hello_params(r:Request) -> HTTP_RESPONSE{
    _OK(format!("Hello {} </h1> by request param</p>", r.params.get("name").unwrap()))
}

fn hello_url_param(r:Request) -> HTTP_RESPONSE {
    _OK( format!("Hello {} by url", r.url_params.get("name").unwrap()))
}


#[test]
fn t(){
   assert_eq!(3,3);
}

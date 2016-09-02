extern crate fcgi;
extern crate libc;

use fcgi::{Request, DefaultRequest};

fn main() {
    println!("isCgi: {}", fcgi::is_cgi());
    fcgi::initialize_fcgi();
    let mut request: DefaultRequest = Request::new().unwrap();
    while request.accept() {
        let uri = request.get_param("REQUEST_URI");
        println!("request uri    {:?}", uri);
        
        let root = request.get_param("DOCUMENT_ROOT");
        println!("document root  {:?}", root);
        
        let script = request.get_param("SCRIPT_NAME");
        println!("script name    {:?}", script);
        
        let method = request.get_param("REQUEST_METHOD");
        println!("request method {:?}", method);
        
        let user = request.get_param("REMOTE_USER");
        println!("remote user    {:?}", user);
        
        let received = request.readall();
        println!("Received (size={})", received.len());
        if received.len() > 0 {
            println!("8<------------------");
            println!("{}", received);
            println!("8<------------------");
        }
        
        let body = "Content-type: text/html\r\n\r\n<header><title>Hello World</title></header>\r\n<body> <h1>Hello World!</h1>  </body>";
        
        let byte_count = request.write(c_type);
        request.error("Test error!");
        println!("number of bytes written {}", byte_count);
        request.flush(fcgi::StreamType::OutStream);
        request.finish();
    }
}

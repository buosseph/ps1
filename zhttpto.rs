//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::os;
use std::io::net::ip::{SocketAddr};
use std::{str};

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut visitor_count:   uint = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;        
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            let split_str: ~[&str] = request_str.split(' ').collect();
            
            let path = os::getcwd();
            let mut path_str: ~str;
            if split_str[0] == "GET" && split_str[1] != "" {
                path_str = 
                    match path.as_str() {
                        Some(string) => string+split_str[1],
                        None => ~"/"
                    };
                let cwdpath = Path::new(path_str.clone());
                let fix = path_str.slice(path_str.len()-5, path_str.len()).to_owned();

                if split_str[1] == "/" {
                    
                    println(format!("Received request :\n{:s}", request_str));
                    
                    let response: ~str = 
                        ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                         <doctype !html><html><head><title>Hello, Rust!</title>
                         <style>body { background-color: #111; color: #FFEEAA }
                                h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                                h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                         </style></head>
                         <body>
                         <h1>Greetings, Krusty!</h1>
                         </body></html>\r\n";
                    stream.write(response.as_bytes());
                    println!("Connection terminates.");
                }

                else if cwdpath.is_file() && fix == ~".html" {
                    println!("File requested: {:s}", path_str);
                    
                    let mut file = buffered::BufferedReader::new(File::open(&cwdpath));
                    let fl_arr: ~[~str] = file.lines().collect();
                    let mut fr = ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
                    for line in fl_arr.iter() {
                        fr = fr + line.to_owned() + "\r\n";
                    }
                    stream.write(fr.as_bytes());
                }
                else {
                    println!("Error reading file. Recieved request :\n{:s}", request_str);
                    let fr = ~"HTTP/1.1 418 I'M A TEAPOT\r\nContent-Type: text/html; charset=UTF-8\r\n\r\nI'm a teapot";
                    stream.write(fr.as_bytes());
                    println!("End of failed request.");
                }
            }
            unsafe {
                    visitor_count+=1;
                    println!("Request count: {:u}\n", visitor_count);
            }
        }
    }
}

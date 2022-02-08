// mod protos;

mod protos;
use protobuf::Message;

// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

// For demonstration we take `GetRequest` messages from a source generated
// with pure-rust codegen, and `GetResponse` with `protoc`-based codegen.
// This is not needed in practice, done here for demonstration purposes.
// use protos::test::TestMsg;
use protos::test::TestMsg;



fn test_proto() {
    let mut msg = TestMsg::new();
    msg.str = "testStr".to_string();
    msg.number = 232;
    

    let msg_bytes: Vec<u8> = msg.write_to_bytes().unwrap();


    let other_msg = TestMsg::parse_from_bytes(&msg_bytes).unwrap();
    let other_str = other_msg.str;
    println!("{}", other_str);
}

fn main() {
    println!("Hello, world!");
    test_proto();
}


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Hello, world!");
//     test_proto();


//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(y) => println!("Matched, y = {:?}", y),
//         _ => println!("Default case, x = {:?}", x),
//     }

//     println!("at the end: x = {:?}, y = {:?}", x, y);


//     let listener = TcpListener::bind("127.0.0.1:8080").await?;

//     loop {
//         let (mut socket, _) = listener.accept().await?;

//         tokio::spawn(async move {
//             let mut buf = [0; 1024];

//             // In a loop, read data from the socket and write the data back.
//             loop {
//                 let n = match socket.read(&mut buf).await {
//                     // socket closed
//                     Ok(n) if n == 0 => return,
//                     Ok(n) => n,
//                     Err(e) => {
//                         eprintln!("failed to read from socket; err = {:?}", e);
//                         return;
//                     }
//                 };

//                 // Write the data back
//                 if let Err(e) = socket.write_all(&buf[0..n]).await {
//                     eprintln!("failed to write to socket; err = {:?}", e);
//                     return;
//                 }
//             }
//         });
//     }
// }

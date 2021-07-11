//cargo new client --bin //binary flag
use std::io::{self, ErrorKind, Read, Write};//self import i/o lib itself (import errorkind, read and write)
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    //create a mutable client which is a tcpstram and connect it with ip LOCAL if fails it will put the msg
    client.set_nonblocking(true).expect("failed to initiate non-blocking");
    //setting our client to be non-blocking if faild it will print the msg
    let (tx, rx) = mpsc::channel::<String>();//instantiate our channels and assign to tx and rx
    thread::spawn(move || loop {//spawnning our thread and create a move closure inside of it with a loop
        let mut buff = vec![0; MSG_SIZE];
        //immediately create a mutable buffer here with 0's inside of it and of size MSG_SIZE
        match client.read_exact(&mut buff) {//match on client.read_exact and read our msg through the buffer
            Ok(_) => {//if we get back OK 
                //we say let our msg=buffer and turn it into an iterator and check if the ref's inside of it ==0(discard whitespaces)
                //we collect all of them in side of our vector 
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message recv {:?}", msg);// we print our that we recieved this msg
                //the server will send back wheather or not a "message recv" and if it did 
                //it'll tell us then we'll send back the msg
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            //we check if the error kind is the type which will block our non-blocking 
            //in that case we will send a unit type if it is.
            Err(_) => {//if any another type of error is encountered then we break out of our loop
                println!("connection with server was severed");//we print it.
                break;
            }
        }
        match rx.try_recv() {//then we'll match on rx.try_recv
        //bascially we want to see if the server sends back a message that it got the msg that we're sending from the client
            Ok(msg) => {//if we do get that msg 
                //then we will clone that msg into buffer converting them to bytes
                let mut buff = msg.clone().into_bytes();
                //then we'll resize our buffer by our MSG_SIZE
                buff.resize(MSG_SIZE, 0);
                //then we write our buffers into our client if they fail we will print out the msg
                client.write_all(&buff).expect("writing to socket failed");
                //otherwise we print message sent and print the msg
                println!("message sent {:?}", msg);
            }, 
            Err(TryRecvError::Empty) => (),//see if our try recieve error is empty and if it is we will send back a unit type
            Err(TryRecvError::Disconnected) => break//if it is a disconnected type we will break the loop
        }

        thread::sleep(Duration::from_millis(100));//make our thread sleep for 100ms after each loop
    });
    println!("Write a Message:");//this shows up when user opens up the client
    loop {//we loop 'cause user can type multiple msg's at same time
        let mut buff = String::new();//create a new mutable string
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");//then we want to read from that string from our std input
        //user types form console read that into our string 
        let msg = buff.trim().to_string();//trim our buffer and convert to string and put in msg variable
        if msg == ":quit" || tx.send(msg).is_err() {break}//we will check on the msg if the msg is equivalent to calling quit
        //or transmitter.send msg comes back with and error we break the loop and print cyu
    }
    println!("cyu !");
}
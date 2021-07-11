use std::io::{ErrorKind, Read, Write};//error type with read and write trait
use std::net::TcpListener;//allow us to create our server and listen on a port
use std::sync::mpsc;//allow us to spawn a channel
use std::thread;//allow us to work with multiple threads

const LOCAL: &str="127.0.0.1:6000";//local host with a port
const MSG_SIZE: usize=32;//buffer size of our messages

fn sleep(){
    thread::sleep(::std::time::Duration::from_millis(100));
    //pass standard time duration from milliseconds 
    //allow our thread to sleep for 100 ms bw each of the loop
}

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");//instantiating our server
    //bind to our local(ip) otherwise return msg
    server.set_nonblocking(true).expect("Failed to initialize non-blocking");//push our server to non-blocking mode
    //if it faild it will show msg in panel
    //non-blocking mode lets our server constantly check for msg's and so on and so forth
    let mut clients = vec![];
    //mutable vector clients that will allow us to put our clients here
    //allow us to have multiple clients to our server at once
    let(tx, rx) = mpsc::channel::<String>();
    //instantiate our channel  then we  assigned it to a string type
    //telling channel that we will send some bunch of strings
    loop{
        if let Ok((mut socket, addr)) = server.accept(){
            //if binding to destruct our result from server.accept(allow us to accept connections to this server)
            println!("Client {} connected", addr);
            //socket here will be a tcp string that's connecting
            //and the address will be the actual socket address
            let tx=tx.clone();
            //cloned our tx(transmitter) 
            clients.push(socket.try_clone().expect("Failed to clone client"));
            //we take our socket and try to clone it and push it to our clients
            //vector if it comes back and fails it will print the msg
            //Why to clone our socket ? so that we can push it into our thread
            thread::spawn(move || loop{//spawnning our thread here with a move closure inside of it
                let mut buff = vec![0; MSG_SIZE];
                //create a mutable buffer a vector with zeros with the MSG_SIZE
                match socket.read_exact(&mut buff){
                    //socket.read_exact will read our msg into our buffer
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        //take the msg that we're receiving convert to iterator then we take all the charachters
                        //that aren't whitespaces and collect them inside of our vector
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                        //we've to convert the strings into an actual string so we will
                        //use utf-8 and if that comes back with error we'll print the msg
                        println!("{}: {:?}", addr, msg);//we will print our the address sent the msg
                        tx.send(msg).expect("failed to send msg to rx");//sends our msg from transmitter to our reveiver
                        //otherwise we print error msg (try-catch block in c++)
                    }, 
                    //we'll check the actual error inside of our error
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    //if our error kind's the type of error we face will block our non-blocking
                    //then we send back a unit type so we just continue otherwise we check for another 
                    //error which could be anything (we don't care about) we print closing connectiion with our client
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;//we break our loop here
                    }
                }
                //problem is that our thread would be constantly looping and it would be really
                //awkward we want something so that our loop to sort of rest when not recieving msg's
                sleep();
            });
        }
        //when our server recieves a msg and we will try to recieve it through a channel
        if let Ok(msg) = rx.try_recv() {
            //set our mutable vector clients to iterator then we will filter through
            //our clients and set the buffer and convert our msg to bytes
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                //then we resize our buffer based on our MSG_SIZE
                buff.resize(MSG_SIZE, 0);
                //take our client and write all of the entire buffer and we're 
                //going to map it into our client and send it back and collect it into a vector
                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();//functional programming
        }
 
        sleep();
    }
}
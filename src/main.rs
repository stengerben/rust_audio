extern crate gio;
extern crate gtk;


use rodio::{Decoder, OutputStream, source::Source};

use std::{thread,
    path::Path,
    time::Duration,
    fs::File,
    io,
    io::{Write, BufReader, BufRead, Error},
    net::{TcpListener, TcpStream},
};
use audiotags::Tag;
use unix_named_pipe::*;
//use rodio::{Decoder, OutputStream, source::Source};

fn main() {
    let music_path = Path::new("/music");
    let mut song = String::from("Zombies_Ate_My_Neighbors.mp3");

    let pipe_dir = Path::new("/tmp");
    let mut pipe = String::from("snapfifo0");

    let pipe_path = pipe_dir.join(&mut pipe.trim());

    //io::stdin()
    //     .read_line(&mut song)
    //     .expect("Failed to read line");
    
    let path = music_path.join(&mut song.trim());
    println!("{}", path.display());

    let tag = Tag::new().read_from_path(&path).unwrap();
    println!("{}", tag.artist().unwrap()); //(title, artist, ) Metadata for song default.mp3

    let mut f = File::open("test/pp.wav").expect("file not found");
    let buffered = BufReader::new(f);

    println!("{}", pipe_path.display());
    let mut fifopipe = File::open(&pipe_path).expect("could not open fifo for writing");
    //CRASHING HERE (COULD NOT OPEN FIFO FOR WRITING OS { code: 6, kind: Uncategorized, message: "No such device or address" } )

    for line in buffered.lines() {
        write!(fifopipe, "{}", line.unwrap());
        println!("{}", fifopipe.is_fifo().unwrap());
    }
    

    /*println!("{}", path.display());
    let (_stream, stream_handle) = OutputStream::try_default().unwrap(); //Figure out error handling
    let file = BufReader::new(File::open(path).unwrap());
    println!("Running");
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples());

     */
    //std::thread::sleep(std::time::Duration::from_secs(30));
    /*
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    //Blocks main thread until handle terminates
    handle.join().unwrap();
    
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    */

}
/*
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    
    println!("Request: {:#?}", http_request);
} */
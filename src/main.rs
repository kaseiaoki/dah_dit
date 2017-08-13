use std::io;
use std::string::String;
use std::env;
use std::vec::Vec;
use std::thread;
use std::process;
use std::mem;
use std::path::PathBuf;
extern crate cpal;
extern crate futures;
use futures::stream::Stream;
use futures::task;
use futures::task::Executor;
use futures::task::Run;
use std::sync::Arc;
use std::time::Duration;

struct MyExecutor;


impl Executor for MyExecutor {
    fn execute(&self, r: Run) {
        r.run();
    }
}
fn morse(word: u8)->&'static str{
    let morse = ["1200","2111","2121","211","1000","1121","2210","1111","1100","1222","2120","1211","2200","2100","2220","1221","2212","1210","1110","2000","1120","1112","1220","2112"];
    let morsey = "2122";
    let morsez = "2211";
    let mut ans = "0";
    let mut w = word as usize;
    match word{
        25 => ans = morsey,
        26 => ans = morsez,
        _ => ans = morse[w],
     }
    return ans;
}
fn aplay(time : Vec<&str>) -> bool{
  let mut code :[u64; 500] = [0; 500];
  let mut i :usize = 0;
  for time in time{
      for t in time.chars(){
          if(t=='1'){
              code[i] = 50;
              print!(". ");
          }else if(t=='2'){
              code[i] = 500;
              print!("_ ");
          }
          i+=1;
      }
  }
  println!("");
    let endpoint = cpal::get_default_endpoint().expect("Failed to get default endpoint");
    let format = endpoint.get_supported_formats_list().unwrap().next().expect("Failed to get endpoint format");

    let event_loop = cpal::EventLoop::new();
    let executor = Arc::new(MyExecutor);

    let (mut voice, stream) = cpal::Voice::new(&endpoint, &format, &event_loop).expect("Failed to create a voice");

    // Produce a sinusoid of maximum amplitude.
    let samples_rate = format.samples_rate.0 as f32;
    let mut data_source = (0u64..).map(move |t| t as f32 * 440.0 * 2.0 * 3.141592 / samples_rate)     // 440 Hz
                                  .map(move |t| t.sin());

    // voice.play();
    task::spawn(stream.for_each(move |buffer| -> Result<_, ()> {
        match buffer {
            cpal::UnknownTypeBuffer::U16(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
                    let value = ((value * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() { *out = value; }
                }
            },

            cpal::UnknownTypeBuffer::I16(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
                    let value = (value * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() { *out = value; }
                }
            },

            cpal::UnknownTypeBuffer::F32(mut buffer) => {
                for (sample, value) in buffer.chunks_mut(format.channels.len()).zip(&mut data_source) {
                    for out in sample.iter_mut() { *out = value; }
                }
            },
        };

        Ok(())
    })).execute(executor);
    thread::spawn(move || {
        for i in 0..code.len(){
            voice.play();
            thread::sleep(Duration::from_millis(code[i]));
            voice.pause();
            thread::sleep(Duration::from_millis(500));
            // voice.play();
        }
        voice.pause();
    });
    event_loop.run();

    return true;
}
fn main() {
    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword);
    let vec: Vec<&str> = keyword.split_whitespace().collect();
    let word = vec[0];
    let no : Vec<&str>;
    let mut i :usize = 0;
    let mut time: Vec<&str> = Vec::new();
    for sig in word.as_bytes() {
       let t= morse(sig-97);
       time.push(&t);
       i+=1;
    }
    aplay(time);
}

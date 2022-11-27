use crate::{RawPuller, Puller};
use std::io;
use std::io::Write;
use async_trait::async_trait;


struct StdoutPuller;
#[async_trait]
impl RawPuller<String> for StdoutPuller{
    async fn pull(&mut self,value: String) {
        let mut writer = io::stdout().lock();
        writer.write_all((&value).as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}

pub fn into_stdout() -> Puller<String>{
    Puller::new(
        StdoutPuller
    )
}

struct StderrPuller;
#[async_trait]
impl RawPuller<String> for StderrPuller{
    async fn pull(&mut self,value: String) {
        let mut writer = io::stderr().lock();
        writer.write_all((&value).as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}


pub fn into_stderr() -> Puller<String>{
    Puller::new(
        StderrPuller
    )
}
use std::{io, process::Command};

use crate::{pipeline, Pipeline};

pub fn command(com: &str) -> Pipeline<(), (), io::Error> {
  let text = com.to_string();
  pipeline(move |_| {
    Command::new(&text).spawn().and_then(|_| Ok(()))
  })
}
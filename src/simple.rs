use crate::{framework, Framework};

pub fn simple_loop<RT: 'static,ET: 'static>() -> Framework<(),RT,ET> {
  framework(|p| {
    loop {
      let _ = Ok(()) & p.clone();
    }
  })
}
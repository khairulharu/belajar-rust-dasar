use crate::layla::tembak_musuh;
use crate::aurora::say_hello_this_is_aurora;

pub fn zilong_speaking() {
     say_hello_this_is_aurora();
     tembak_musuh();
     println!("this is zilong speaking, nice to meet you guys");
}

pub mod hero {
     pub mod skill {
          pub fn tembak_musuh() {
               println!("this");
               crate::zilong::zilong_speaking();
               println!("this");
               super::super::zilong_speaking();
          }
     }
}
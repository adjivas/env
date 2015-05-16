// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/env/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate envlib;

fn main() {
  let mut line:String = String::new();
  loop {
    match std::io::stdin().read_line(
      &mut line
    ) {
      Ok(_) => {
        match std::io::Write::write(
          &mut std::io::stderr(),
          &envlib::env::interpreter(&line.chars().take_while(|x|
            *x != '\n'
          ).collect()).into_bytes()
        ) {
          Ok(_) => line.clear(),
          Err(why) => panic!("Unable to write to stderr: {}", why),
        }
      },
      Err(_) => break ,
    }
    line.clear();
  }
}

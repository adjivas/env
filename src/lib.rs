// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/env/LICENCE.
//
// This file may not be copied, modified, or distributed
// except according to those terms.

pub mod env {
  use std::env;

  /// The `get_vars` function returns all variables from environement.
  fn get_vars (
  ) -> String {
    let mut envs:String = String::new();

    for env in env::vars() {
      let (key, val) = env;

      envs.push_str(&key);
      envs.push_str(" -> ");
      envs.push_str(&val);
      envs.push_str("\n");
    }
    envs
  }

  /// The `get_var` function returns the variable according to key.
  fn get_var (
    key: String,
  ) -> String {
    match env::var(&key) {
      Ok(val) => val,
      Err(_) => "".to_string(),
    }
  }

  /// The `del_var` function removes the variable according to key.
  fn del_var (
    key: String,
  ) -> String  {
    env::remove_var(&key);
    key
  }

  /// The `set_var` function writes the variable according to key.
  fn set_var (
    key: String,
    val: String,
  ) -> String {
    env::set_var(&key, &val);
    val
  }

  /// The `replace` function returns a new line replaced by
  /// all value from key's environement.
  pub fn replace (
    line: &String,
  ) -> String {
    let mut result:String = line.clone();

    for env in env::vars() {
      let (key, val) = env;

      match line.contains(&key) {
        true => result = line.replace(&key, &val),
        false => continue ,
      }
    }
    result
  }

  /// The `interpreter` function chooses the action.
  pub fn interpreter (
    line: &String,
  ) -> String {
    let mut args = line.split(" -> ");

    match args.next() {
      Some(key) => {
        match args.next() {
          Some(val) => match val.is_empty() {
            true => del_var(key.to_string()),
            false => set_var(key.to_string(), val.to_string()),
          },
          None => match key.is_empty() {
            true => get_vars(),
            false => get_var(key.to_string()),
          },
        }
      },
      None => panic!("env interpreter fail"),
    }
  }
}

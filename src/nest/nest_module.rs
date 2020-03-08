use super::super::server;

pub fn test_func() {
  println!("Nest module function.");
  server::connect();
}

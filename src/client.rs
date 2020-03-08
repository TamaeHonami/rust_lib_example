use super::nest::inner_call_module as calling;

pub fn connect() {
  println!("Client connect!!");
  calling::call();
}

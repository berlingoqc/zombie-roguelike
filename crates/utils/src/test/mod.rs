#[macro_export]
macro_rules! get_crate_root_path{($fname:expr) => (
    concat!(env!("CARGO_MANIFEST_DIR"), "/", $fname) // assumes Linux ('/')!
  )}
  
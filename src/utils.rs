/// Prints the Ferris crab ASCII art with JasDB branding
pub fn print_ferris() {
    let version = env!("CARGO_PKG_VERSION");
    println!(
r#"
     _~^~^~_
 \) /  o o  \ (/
  ' _   u   _ '
   \ '-----' /
      JasDB
 Powered by Rust!

 https://github.com/DarthData410/jasdb
 v{}
"#,
        version
    );
}
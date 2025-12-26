use std::fs::File;
fn main() {
   // panic!("Crash and burn");
   let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
         Ok(file) => file,
         Err(error) => match error.kind() {
             std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                 Ok(fc) => fc,
                 Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
             },
             _ => {
                 panic!("There was a problem opening the file: {:?}", error)
             }
         },
    };


}


use std::io;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {

    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}
//如果f.read_to_string 返回正常会返回Ok(f)的值给f,否则提前把Err的错误返回给给调用者
fn read_file_str () -> Result<String,io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;//这个也是如果read_to_string的值返回正常给f,如果不是将提前返回Err把值返回给调用者
    Ok(s)
}

fn read_file_1_str () -> Result<String,io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_2_str() -> Result<String,io::Error> {
    fs::read_to_string("hello.txt");
}







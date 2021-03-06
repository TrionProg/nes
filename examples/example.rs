//This is an example, how to work with NES
#![feature(box_patterns)]

#[macro_use]
extern crate nes;
use nes::{ErrorInfo,ErrorInfoTrait};

define_error!( ReadFileError,
    IOError(io_error:Box<std::io::Error>) => "IO Error: {}",
    ReadFileError(io_error:Box<std::io::Error>, file:String ) => "Can not read file \"{2}\" : {1}" //1,2 is order of args, note:0 is ErrorInfo
);

define_error!( CommonError,
    ReadFileError(read_file_error:Box<ReadFileError>) => "read file error {}",
    NoArguments() => "no arguments",
    IncorrectExtension(file_name:String, extension:String) => "Expected extension \"{2}\" for file \"{1}\""
);

impl_from_error!(ReadFileError => CommonError);

fn process() -> result![CommonError] { //Or Result<CommonError>
    let file_name=read_arg()?;

    let lines=read_file(file_name)?;

    for line in lines.iter() {
        print!("L:{}",line);
    }

    ok!()
}

fn read_arg() -> result![String,CommonError] {//Or Result<String,CommonError>
    let mut args=std::env::args();
    args.next();

    let file_name=match args.next() {
        Some( file_name ) => file_name,
        None => return err!(CommonError::NoArguments),
    };

    if !file_name.ends_with(".rs") {
        return err!(CommonError::IncorrectExtension, file_name, ".rs".to_string())
    }

    ok!(file_name)
}

fn read_file(file_name:String) -> result![Vec<String>,ReadFileError] {//Or Result<Vec<String>,ReadFileError>, but then where are too much <<>>
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file=try!( File::open(file_name.as_str()), ReadFileError::ReadFileError, file_name );

    let mut buf_reader = BufReader::new(file);
    let mut lines=Vec::new();
    let mut line=String::with_capacity(80);

    loop {
        match try!( buf_reader.read_line(&mut line), ReadFileError::IOError ) {
            0 => break,
            _ => lines.push(line.clone()),
        }

        line.clear();
    }

    ok!(lines)
}

fn main() {
    match process() {
        Ok(_) => {},
        Err(CommonError::IncorrectExtension(_,file_name, extension)) => println!("incorrect extension {}",extension),
        Err(e) => {
            match e {
                CommonError::ReadFileError(_, box ReadFileError::ReadFileError(_,ref io_error, ref file)) => println!("can not read file \"{}\"",file),
                _ => {println!("{}",e)} //or println!("{:?}",e)
            }
        }
    }
}

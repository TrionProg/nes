# NES
New Error System is the library for rust, that makes the syntax more elegant for operating by errors.

Description
-----------
* It uses stable version of rust
* Each error stores the location in source code, where the error has been occurred, because some errors like std::io::Error may occurs in different places in code, it is useful for detection of problems.
* Where is collection of macros that make the syntax more elegant and short.
* You can use your own ErrorInfo, that stores information where an error has been occurred.

If you have some ideas, write them in Issues.

Usage
-----

Cargo.toml
```
nes = "*"
```

[Documentation](https://docs.rs/nes/0.1.0/nes/)
[Example](https://github.com/trionprog/nes/examples/example.rs)

```
//See examples/example.rs

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

fn process() -> result![CommonError] {
    let lines=read_file("file.rs")?;

    for line in lines.iter() {
        print!("L:{}",line);
    }

    ok!()
}

fn read_file(file_name:String) -> result![Vec<String>,ReadFileError] {
    use std::io::prelude::*;

    let file=try!( std::fs::File::open(file_name.as_str()), ReadFileError::ReadFileError, file_name );

    let mut buf_reader = std::io::BufReader::new(file);
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
```

License
-------

MIT

//!NES - New Error System is the library for rust, that makes the syntax more elegant for operating by errors.
//!Each error stores the location in source code, where the error has been occurred, because some errors like std::io::Error may occurs in different places in code, so it is useful for detection of problems.
//!Note, that this errors are useful for users or sysadmins. panic!() is preferable for developers to detect bugs. For example, server can not be started because port 80 is busy. In this case you should show user the window,
//!that contains simple information and print to log more detailed information.
//!
//!You can match errors:
//!
//! # Example
//!
//! ```
//!#![feature(box_patterns)]
//!
//!match process() {
//!    Ok(_) => {},
//!    Err(CommonError::IncorrectExtension(_,file_name, extension)) => println!("incorrect extension {}",extension),
//!    Err(e) => {
//!        match e {
//!            //_, is error_info that contains information, where the error has been occurred, we skip it
//!            CommonError::ReadFileError(_, box ReadFileError::ReadFileError(_,ref io_error, ref file)) => println!("can not read file \"{}\"",file),
//!            _ => {println!("{}",e)} //or println!("{:?}",e)
//!        }
//!    }
//!}
//! ```
//!
//!
//! By `println!("{}",e)` You will get error like(not in case above):
//!
//! ```text
//!example/examples/example.rs 16:0   //line, where impl_from_error!() is.
//!read file error example/examples/example.rs 51:13    //line where thr error has been occurred
//!Can not read file "no_file.rs" : No such file or directory (os error 2)    //description of error
//! ```
//!
//!Do not forget to see examples directory


///This is standard ErrorInfo structure.
pub struct ErrorInfo {
    file:&'static str,
    line:u32,
    col:u32
}

///You should implement this trait for your own ErrorInfo, then you need, for example, get current time and write to log in method new.
///
/// # Example
///
/// ```
///use nes::ErrorInfoTrait; //Do not include ErrorInfo! Use your own ErrorInfo instead standard.
///
///pub struct ErrorInfo {
///    file:&'static str,
///    line:u32,
///    col:u32,
///    //some fields
///}
///
///impl ErrorInfoTrait for ErrorInfo {
///    fn new(file:&'static str, line:u32, col:u32 ) -> Self{
///        ErrorInfo {
///            file,
///            line,
///            col,
///            //some fields
///        }
///    }
///
///    fn file(&self) -> &'static str { self.file }
///    fn line(&self) -> u32 { self.line }
///    fn col(&self) -> u32 { self.col }
///}
/// ```
///

pub trait ErrorInfoTrait: std::fmt::Display{
    fn new(file:&'static str, line:u32, col:u32 ) -> Self;

    fn file(&self) -> &'static str;
    fn line(&self) -> u32;
    fn col(&self) -> u32;
}

impl ErrorInfoTrait for ErrorInfo {
    fn new(file:&'static str, line:u32, col:u32 ) -> Self{
        ErrorInfo {
            file,
            line,
            col
        }
    }

    fn file(&self) -> &'static str { self.file }
    fn line(&self) -> u32 { self.line }
    fn col(&self) -> u32 { self.col }
}

impl std::fmt::Display for ErrorInfo{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}:{}", self.file,self.line,self.col)
    }
}

///This macro defines the error.
///
/// # Example
///
/// ```
///define_error!( ReadFileError,
///    IOError(io_error:Box<std::io::Error>) =>
///        "IO Error: {}",
///    ReadFileError(io_error:Box<std::io::Error>, file:String ) =>
///        "Can not read file \"{2}\" : {1}" //1,2 is order of args, note:0 is ErrorInfo
///);
///
///define_error!( CommonError,
///    ReadFileError(read_file_error:Box<ReadFileError>) =>
///        "read file error {}",
///    NoArguments() =>
///        "no arguments",
///    IncorrectExtension(file_name:String, extension:String) =>
///        "Expected extension \"{2}\" for file \"{1}\""
///);
/// ```
///
///You must push other errors in Box. This prevent results that have large size or infinite(if error is recursive).
///In this case Box<..> must be written first, and may be accessed by index like {2}, but index 0 has ErrorInfo, that describes where the error has been occurred.
///
///This macro generates code like
///
/// ```text
///pub enum ReadFileError {
///    IOError(ErrorInfo, Box<std::io::Error>),
///    ReadFileError(ErrorInfo, Box<std::io::Error>, String )
///);
///
///impl ReadFileError {
///    pub fn get_error_info(&mut self) -> &ErrorInfo { ... }
///}
///
///impl std::fmt::Display for ReadFileError {
///    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
///        match *self {
///            ReadFileError::ReadFileError(ref error_info, ref io_error, ref file) => write!(f, "{}\n,Can not read file \"{2}\" : {1}", error_info, io_error, file),
///        }
///    }
///}
///
///impl std::fmt::Debug for ReadFileError { ... } //Short description.
/// ```
///

#[macro_export]
macro_rules! define_error{
    ( $error_name:ident,
        $(
            $var_name:ident ( $( $field_name:ident : $field_type:ty ),* ) => $message:expr
        ),*
    ) => {
        pub enum $error_name {
            $(
                $var_name( ErrorInfo, $( $field_type ),* )
            ),*
        }

        impl $error_name {
            pub fn get_error_info(&mut self) -> &ErrorInfo{
                match *self {
                    $(
                        $error_name::$var_name( ref mut error_info, .. ) => error_info
                    ),*
                }
            }
        }

        impl std::fmt::Display for $error_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $(
                        $error_name::$var_name( ref error_info, $( ref $field_name ),* ) =>
                            write!(f, concat!("{}\n",$message), error_info, $( $field_name ),* )
                    ),*
                }
            }
        }

        impl std::fmt::Debug for $error_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $(
                        $error_name::$var_name( ref error_info, $( ref $field_name ),* ) =>
                            write!(
                                f,
                                concat!("{}\n",stringify!($error_name),"::",stringify!($var_name)," ",$( concat!(stringify!($field_name),":{:?} ") ),* ),
                                error_info, $( $field_name ),*
                            )
                    ),*
                }
            }
        }
/* I think, this is not necessary
        impl Error for $error_name {
            fn description(&self) -> &str {
                match *self {
                    $(
                        $error_name::$var_name( ref error_info, $( ref $field_name ),* ) => concat!(stringify!($error_name),"::",stringify!($var_name))
                    ),*
                }
            }

            fn cause(&self) -> Option<&error> {
                match *self {
                    $(
                        $error_name::$var_name(..) => self.$var_name()
                    ),*
                }
            }
        }
*/
    };

}

///This macro implements From trait for other errors.
///
///It allows you to convert other errors into current and write something like function(..)?.
///Note, that you must use this macro only for errors, that have been defined with define_error!(). For errors like std::io::Error you must use try!() macro, because it gets information, where this error has been occurred.
///
/// # Example
///
/// ```
///impl_from_error!(ReadFileError => CommonError);
///
///fn read_file(file_name:String) -> result![Vec<String>,ReadFileError] { ... }
///
///fn process() -> result![CommonError] { //Or Result<CommonError>
///    let lines=read_file(file_name)?;
///    ...
///}
/// ```
///
/// You will get error like:
///
/// ```text
///example/examples/example.rs 16:0   //line, where impl_from_error!() is.
///read file error example/examples/example.rs 51:13    //line where the error has been occurred
///Can not read file "no_file.rs" : No such file or directory (os error 2)    //description of error
/// ```
///
///Where is second form
///
/// # Example
///
/// ```rust
///impl_from_error!(::module::ReadFileError => CommonError::CanNotReadFile);
/// ```
///


#[macro_export]
macro_rules! impl_from_error{
    ( $from_error:ident => $to_error:ident ) => {
        impl From<$from_error> for $to_error {
            fn from(from_error:$from_error) -> Self {
                $to_error::$from_error(error_info!(),Box::new(from_error))
            }
        }
    };
    ( $from_error:path => $to_error:ident :: $to_variant:ident ) => {
        impl From<$from_error> for $to_error {
            fn from(from_error:$from_error) -> Self {
                $to_error::$to_variant(error_info!(),Box::new(from_error))
            }
        }
    };
}

///This macro generates error that gets information, where the error has been occurred. You should return it.
///
/// # Example
///
/// ```
///let file_name=match args.next() {
///    Some( file_name ) => file_name,
///    None => return err!(CommonError::NoArguments),
///};
///
///if !file_name.ends_with(".rs") {
///    return err!(CommonError::IncorrectExtension, file_name, ".rs".to_string())
///}
/// ```
///

#[macro_export]
macro_rules! err{
    ( $error:path ) => {
        Err(
            $error( error_info!() )
        )
    };
    ( $error:path, $( $arg:expr ),* ) => {
        Err(
            $error( error_info!(), $( $arg, )* )
        )
    };
}

///This macro creates error that gets information, where the error has been occurred. You can insert it into other error.
///
/// # Example
///
/// ```
///let error=Box::new(create_err!(handler::Error::BrockenChannel));
///return err!(Error::HandlerThreadCrash, error, ThreadSource::Handler);
/// ```
///

#[macro_export]
macro_rules! create_err{
    ( $error:path ) => {
        $error( error_info!() )
    };
    ( $error:path, $( $arg:expr ),* ) => {
        $error( error_info!(), $( $arg, )* )
    };
}

///This macro looks like standard try!() macro but it gets information where the error has been occurred.
///
///Note: if error, that you convert to other, contains ErrorInfo(is defined by define_error!() and is not like std::io::Error), you should use ?.
///
/// # Example
///
/// ```
///let file=try!( std::fs::File::open(file_name.as_str()), ReadFileError::ReadFileError, file_name );
///
///match try!( buf_reader.read_line(&mut line), ReadFileError::IOError ) { ... }
/// ```
///

#[macro_export]
macro_rules! try{
    ( $o:expr, $error:path ) => {
        match $o {
            Ok( ok ) => ok,
            Err(e) => {
                return Err(
                    $error( error_info!(), Box::new(e) )
                )
            }
        }
    };
    ( $o:expr, $error:path, $( $arg:expr ),* ) => {
        match $o {
            Ok( ok ) => ok,
            Err(e) => {
                return Err(
                    $error( error_info!(), Box::new(e), $( $arg, )* )
                )
            }
        }
    };
}

///This macro avoids overabundance of <<>> and makes a syntax more beautiful.
///
/// # Example
///
/// ```
///fn process() -> result![CommonError] { ... }
///
///fn read_file(file_name:String) -> result![Vec<String>,ReadFileError] { ... }
/// ```
///

#[macro_export]
macro_rules! result{
    [ $error:ty ] => {
        Result<(), $error>
    };
    [ $ok:ty , $error:ty ] => {
        Result<$ok, $error>
    };
}

///This macro makes a syntax more beautiful.
///
/// # Example
///
/// ```
///ok!() // instead Ok(())
///
///ok!(lines) // instead Ok(lines)
/// ```
///

#[macro_export]
macro_rules! ok{
    () => {
        Ok(())
    };
    ($( $x:expr ),* ) => {
        Ok( $( $x, )* )
    }
}

///This macro returns file,line,column, where an error has been occurred
///

#[macro_export]
macro_rules! error_info {
    () => {
        ErrorInfo::new(concat!(module_path!(),"/",file!()), line!(), column!())
    };
}


///This macro helps to lock mutex and returns error if it is poisoned(second thread has locked the Mutex and panicked).
///
///Where are 4 forms:
///
///`let guard=mutex_lock(mutex)` returns "Error::Poisoned"
///
///`let guard=mutex_lock(mutex,ErrorName)` returns "ErrorName::Poisoned"
///
///`let guard=mutex_lock(mutex,ErrorName::Variant)` returns "ErrorName::Variant"
///
///`let guard=mutex_lock(mutex,ErrorName::Variant,arg1,arg2,...)` returns "ErrorName::Variant(arg1,arg2,...)"
///

#[macro_export]
macro_rules! mutex_lock{
    ( $mutex:expr ) => {
        match $mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return err!(Error::Poisoned),
        }
    };
    ( $mutex:expr, $error:ident ) => {
        match $mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return err!($error::Poisoned),
        }
    };
    ( $mutex:expr, $error:path ) => {
        match $mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return err!($error),
        }
    };
    ( $mutex:expr, $error:path, $( $arg:expr ),* ) => {
        match $mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return Err( $error( error_info!(), $( $arg, )* ) ),
        }
    };

    ( $mutex:expr => $var:ident) => {
        let mut guard=match $mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return err!(Error::Poisoned),
        };

        let $var=guard.deref_mut();
    };
    ( $mutex:expr => $var:ident, $error:ident ) => {
        let mut guard=match $mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return err!($error::Poisoned),
        };

        let $var=guard.deref_mut();
    };
    ( $mutex:expr => $var:ident, $error:path ) => {
        let mut guard=match $mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return err!($error),
        };

        let $var=guard.deref_mut();
    };
    ( $mutex:expr => $var:ident, $error:path, $( $arg:expr ),* ) => {
        let mut guard=match $mutex.lock() {
            Ok(guard) => guard,
            Err(_) => return Err( $error( error_info!(), $( $arg, )* ) ),
        };

        let $var=guard.deref_mut();
    };
}

///This macro helps to lock rw_lock(calls write) and returns error if it is poisoned(second thread has locked the RwLock and panicked).
///
///Where are 4 forms:
///
///`let guard=rw_write(rw_lock)` returns "Error::Poisoned"
///
///`let guard=rw_write(rw_lock,ErrorName)` returns "ErrorName::Poisoned"
///
///`let guard=rw_write(rw_lock,ErrorName::Variant)` returns "ErrorName::Variant"
///
///`let guard=rw_write(rw_lock,ErrorName::Variant,arg1,arg2,...)` returns "ErrorName::Variant(arg1,arg2,...)"
///

#[macro_export]
macro_rules! rw_write{
    ( $rw:expr ) => {
        match $rw.write() {
            Ok(guard) => guard,
            Err(_) => return err!(Error::Poisoned),
        }
    };
    ( $rw:expr, $error:ident ) => {
        match $rw.write() {
            Ok(guard) => guard,
            Err(_) => return err!($error::Poisoned),
        }
    };
    ( $rw:expr, $error:path ) => {
        match $rw.write() {
            Ok(guard) => guard,
            Err(_) => return err!($error),
        }
    };
    ( $rw:expr, $error:path, $( $arg:expr ),* ) => {
        match $rw.write() {
            Ok(guard) => guard,
            Err(_) => return Err( $error( error_info!(), $( $arg, )* ) ),
        }
    };
}

///This macro helps to lock rw_lock(calls read) and returns error if it is poisoned(second thread has locked the RwLock and panicked).
///
///Where are 4 forms:
///
///`let guard=rw_read(rw_lock)` returns "Error::Poisoned"
///
///`let guard=rw_read(rw_lock,ErrorName)` returns "ErrorName::Poisoned"
///
///`let guard=rw_read(rw_lock,ErrorName::Variant)` returns "ErrorName::Variant"
///
///`let guard=rw_read(rw_lock,ErrorName::Variant,arg1,arg2,...)` returns "ErrorName::Variant(arg1,arg2,...)"
///

#[macro_export]
macro_rules! rw_read{
    ( $rw:expr ) => {
        match $rw.read() {
            Ok(guard) => guard,
            Err(_) => return err!(Error::Poisoned),
        }
    };
    ( $rw:expr, $error:ident ) => {
        match $rw.read() {
            Ok(guard) => guard,
            Err(_) => return err!($error::Poisoned),
        }
    };
    ( $rw:expr, $error:path ) => {
        match $rw.read() {
            Ok(guard) => guard,
            Err(_) => return err!($error),
        }
    };
    ( $rw:expr, $error:path, $( $arg:expr ),* ) => {
        match $rw.read() {
            Ok(guard) => guard,
            Err(_) => return Err( $error( error_info!(), $( $arg, )* ) ),
        }
    };
}

///This macro sends a message into channel and returns error if channel is brocken(second thread has panicked or finished).
///
///Where are 4 forms:
///
///`channel_send(channel,message)` returns "Error::BrockenChannel"
///
///`channel_send(channel,message,ErrorName)` returns "ErrorName::BrockenChannel"
///
///`channel_send(channel,message,ErrorName::Variant)` returns "ErrorName::Variant"
///
///`channel_send(channel,message,ErrorName::Variant,arg1,arg2,...)` returns "ErrorName::Variant(arg1,arg2,...)"
///

#[macro_export]
macro_rules! channel_send{
    ( $channel:expr, $message:expr ) => {
        if $channel.send( $message ).is_err() {
            return err!(Error::BrockenChannel)
        }
    };
    ( $channel:expr, $message:expr, $error:ident ) => {
        if $channel.send( $message ).is_err() {
            return err!($error::BrockenChannel)
        }
    };
    ( $channel:expr, $message:expr, $error:path ) => {
        if $channel.send( $message ).is_err() {
            return err!($error)
        }
    };
    ( $channel:expr, $message:expr, $error:path , $( $arg:expr ),* ) => {
        if $channel.send( $message ).is_err() {
            return Err( $error( error_info!(), $( $arg, )* ) )
        }
    };
}

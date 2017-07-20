
pub struct ErrorInfo {
    file:&'static str,
    line:u32,
    col:u32
}

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
/*
//!This macro defines the error, for example
//!
//! ```text
//! let tokens = quote! {

//! };
//! ```
//!
//!

*/

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

    };

}

#[macro_export]
macro_rules! impl_from_error{
    ( $from_error:ident => $to_error:ident ) => {
        impl From<$from_error> for $to_error {
            fn from(from_error:$from_error) -> Self {
                $to_error::$from_error(ErrorInfo::new(concat!(module_path!(),"/",file!()), line!(), column!()),Box::new(from_error))
            }
        }
    };
}

#[macro_export]
macro_rules! err{
    ( $error:path ) => {
        Err(
            $error( ErrorInfo::new(concat!(module_path!(),"/",file!()), line!(), column!()) )
        )
    };
    ( $error:path, $( $arg:expr ),* ) => {
        Err(
            $error( ErrorInfo::new(concat!(module_path!(),"/",file!()), line!(), column!()), $( $arg, )* )
        )
    };
}

#[macro_export]
macro_rules! try{
    ( $o:expr, $error:path ) => {
        match $o {
            Ok( ok ) => ok,
            Err(e) => {
                return Err(
                    $error( ErrorInfo::new(concat!(module_path!(),"/",file!()), line!(), column!()), Box::new(e) )
                )
            }
        }
    };
    ( $o:expr, $error:path, $( $arg:expr ),* ) => {
        match $o {
            Ok( ok ) => ok,
            Err(e) => {
                return Err(
                    $error( ErrorInfo::new(concat!(module_path!(),"/",file!()), line!(), column!()), Box::new(e), $( $arg, )* )
                )
            }
        }
    };
}

#[macro_export]
macro_rules! result{
    [ $error:ty ] => {
        Result<(), $error>
    };
    [ $ok:ty , $error:ty ] => {
        Result<$ok, $error>
    };
}

#[macro_export]
macro_rules! ok{
    () => {
        Ok(())
    };
    ($( $x:expr ),* ) => {
        Ok( $( $x, )* )
    }
}

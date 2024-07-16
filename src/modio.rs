use std::fmt::Debug;
use std::str::FromStr;

#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {{
        use std::io::Write;  
        print!($($arg)*);
        std::io::stdout().flush().unwrap();
    }};
}

pub fn input_num<T: FromStr>() -> T
    where T::Err: Debug 
{
    let mut num = String::new();

    std::io::stdin().read_line(&mut num).unwrap();
    num.trim().parse().expect("Not a number")
}
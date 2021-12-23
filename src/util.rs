use std::fs;
use std::str::FromStr;

pub fn read_file_to_string(file: &str) -> String {
    fs::read_to_string(file).expect(&format!("Couldn't open {}", file)[..])
}

pub fn read_file_to_string_iter(file: &str) -> impl Iterator<Item = String> {
    read_file_to_vec::<String>(file).into_iter()
}

pub fn read_file_to_vec<T>(file: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    read_file_to_string(file)
        .split('\n')
        .into_iter()
        .map(|x| x.parse::<T>().expect("Couldn't parse value in file"))
        .collect()
}

pub fn sprint<T>(x: T)
where
    T: std::fmt::Display,
{
    println!("{}", x)
}

pub fn dprint<T>(x: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", x)
}

pub trait UnsafeParseable {
    fn unsafe_parse<T>(self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug;
}

impl UnsafeParseable for &str {
    fn unsafe_parse<T>(self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.parse::<T>().expect("Couldn't parse value")
    }
}

pub fn bin_string_to_i32(s: &String) -> i32 {
    i32::from_str_radix(&s, 2).expect("Couldn't convert from String to i32")
}

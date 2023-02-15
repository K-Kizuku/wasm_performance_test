// #[no_mangle]
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
#[macro_use]
extern crate lazy_static;

use std::os::raw::c_char;
// use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::fmt::Write;
#[no_mangle]
pub fn performance(n: i32) -> i32 {
    // let mut x  = "non".to_string();
    // if n == 10000 {

    //     x = "done".to_string();
    // }
    // x
    let mut i = 0;
    loop {
        i += 1;
    
        if i == n {
          return 32;
        }
      }
}


static HEAP_STRING: Mutex<String> = Mutex::new(String::new());
#[no_mangle]
pub extern "C" fn spigot_pi() -> *const c_char {

    let base:i64=10000;
    let mut n:i64 = 8400;
    let mut i;
    let mut out=0;
    let mut denom;
    let mut res = String::from("");


    let mut numerator: [i64;8400]=[base/5;8400];

    while n > 0 {
        let mut temp=0;
        i=n-1;
        while i>0{
            denom=2*i-1;
            temp = temp*i+numerator[i as usize]*base;
            numerator[i as usize]=temp%denom;
            temp = temp/denom;
            i-=1;
        }
        res.push_str(&format!("{:>04}",out+temp/base));
        out = temp % base;
        n -= 14;
    };
    let mut output = HEAP_STRING.lock().unwrap();
    output.clear();
    write!(output, "{}\0", res).unwrap();
    output.as_ptr() as *const c_char
}


#[no_mangle]
pub fn temp_pi2() -> &'static str {

    let base:i64=10000;
    let mut n:i64 = 8400;
    let mut i;
    let mut out=0;
    let mut denom;
    let mut res = String::from("");


    let mut numerator: [i64;8400]=[base/5;8400]; //分子の初期化

    while n > 0 {
        let mut temp=0;
        i=n-1;
        while i>0{
            denom=2*i-1;
            temp = temp*i+numerator[i as usize]*base;
            numerator[i as usize]=temp%denom;
            temp = temp/denom;
            i-=1;
        }
        res.push_str(&format!("{:>04}",out+temp/base));
        out = temp % base;
        n -= 14;
    };
    let res = "u";
    res
    // // let res: String = "rtring from rust".to_string();
    // let mut output = HEAP_STRING.lock().unwrap();
    // output.clear();
    // write!(output, "{}\0", res).unwrap();
    // output.as_ptr() as *const c_char
}
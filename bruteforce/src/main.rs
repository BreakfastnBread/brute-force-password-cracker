use std::{thread, time};

fn main() {
    bruteforce(gen_alpha());
}

pub fn gen_alpha() -> Vec<char> {
    let mut alphabet: Vec<char> = vec!['\0'];
    let mut alphabet1 = (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();
    let alphabet2 = (b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();
    for i in alphabet2 {
        alphabet1.push(i);
    }
    for i in alphabet1 {
        alphabet.push(i)
    }
    let symbols: Vec<char> = vec!['@', '!', '#', '$', '%', '&', '*','_', '-'];
    for i in symbols {
        alphabet.push(i);
    }
    let numerals: Vec<char> = vec!['1','2','3','4','5','6','7','8','9',];
    for i in numerals {
        alphabet.push(i);
    }
    return alphabet;
}

fn bruteforce(chars: Vec<char>) {
    for a in &chars {
        for b in &chars {
        for c in &chars {
        for d in &chars {
        for e in &chars {
        for f in &chars {
        for g in &chars {
        for h in &chars {
        for i in &chars {
            let x = format!("{}{}{}{}{}{}{}{}{}",a,b,c,d,e,f,g,h,i);
            println!("{}", x);
        }}}}}}}}}
}

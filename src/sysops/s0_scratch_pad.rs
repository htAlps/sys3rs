Γ0․s0_scratchpad.rs․          ᚖ․Catg․Tags: CODE, KB
․․ο․𐍂𐓙f𐓙εl․𐡇․𐒱lpíz𐓙r․         ᚖ․Subj․Tags: Code Fragments to Support Associated Module 
·   ·                           ·

Contents:

  № Nomenclature, Glossary, Staging § Footer․

Anouncements:


•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․9․⛩κ✨λ Short Programs § Code Fragments  (Rev․ Chrono)

․4․ PREAMBLE:

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
❓𐤒 QaaS ─ Questions Answers Actions § Searches

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
🔭 URLs →

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
⭐ν NOTES:


•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․7․κλ Short Programs (Rev․ Chrono)


•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
․5․ 



•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
․1․ End of
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·

                                                                 ·════ 🔻 ════·
                                                                       💥
                                                                 ·════ 🔻 ════·

·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․7․κλ Short Programs (Rev․ Chrono)


•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
․5․λ 



•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
•           ·                   ·                   ·                   ·                   ·                   ·                   ·            • 
․1․ End of
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·

                                                                 ·════ 🔻 ════·
                                                                       💥
                                                                 ·════ 🔻 ════·

·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․5․λ sys3rs::main  The Code Pit § Fragments 


•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
․5․λ✧23․01․04․ The Code Pit: sys3rs::main

/ *
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ main is the overall framework's executor for sys3rs; it selects a module from below and runs it's exec fn
fn main() -> Result<(), String> {

    let my_location = "main"; 
    print!("\n🎡𐡋 running: {}\n", my_location);
    match lib3::check() {                                          // temporarily checking library 
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ main is the overall framework's executor for sys3rs; it selects a module from below and runs it's exec fn
fn main() -> Result<(), String> {

    let my_location = "main"; 
    match sysops::s1_exec::run() {
        Err(ee) => Err(format!("Trace: {ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
λ Crates § Modules

use std::error::Error;

mod lib3;
mod sysops;
        _       => match sysops::check() {
mod rsx0;
        _       => match rsx0::check() {
mod rsx1;
        _       => match rsx0::check() {
mod rsx2;
        _       => match rsx0::check() {
mod rsx3;
        _       => match rsx0::check() {

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
OLDER 
use lib3::q1_lex::{prt_cmd};
use lib3::q2_hash::{check_regex_helpers};
use lib3::q3_regex::{check_regex_helpers};

mod rsx0;       use rsx0::{check};

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("🎡𐡋 {my_location} \n");
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
* /
End Of The Code Pit




•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
․1․ End of
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·

                                                                 ·════ 🔻 ════·
                                                                       💥
                                                                 ·════ 🔻 ════·

·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․7․κλ Short Programs (Rev․ Chrono)


•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
․5․λ Code Pit Fragments  


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
FROM REGEX

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("\n🎡𐡋 removing (,\" ... \",) within single lines in a file \n");
    let loaded_string = fs::read_to_string("/usr/local/sys/rust/data/x1_data.csv").expect("check::Read Error");
    let parsed_string = remove_1_inline_quotes(loaded_string.clone());
    fs::write("/usr/local/sys/rust/data/q11_p_data.csv", &parsed_string).expect("check::Write Parse1 Error");

    print!("\n🎡𐡋 removing (,\" ... \",) across multiple lines in a file \n");
    let parsed_string = remove_2_multiline_quotes(loaded_string);
    fs::write("/usr/local/sys/rust/data/q12_p_data.csv", &parsed_string).expect("check::Write Parse4 Error");

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
fn prt_map(map: &HashMap<&str, i32>) -> String {
    print!("    HashMap: [ ");
    for (kk, vv) in map {
        print!("({}, {}), ", kk, vv);
    }
    print!("]\n");
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// count_str counts words in a String object
fn count_str(ss: &String) -> HashMap<&str, i32> {
    let mut res: HashMap<&str, i32> = HashMap::new();
    for txt in ss.split_whitespace() {
        let p_count: &mut i32 = res.entry(txt).or_insert(0);
        *p_count += 1;          // ⭐ pretty cool to change entry-values via a mutable pointer to our count value
    }
    res
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let re = Regex::new(r"(?P<y>\d*)-(?P<m>\d*)-(?P<d>\d*)").unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let after = re.replace_all(before, "$m/$d/$y");
    print!("before: {before}\n");
    print!("after:  {after}\n");

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let re = Regex::new(r"(?m)^line \d+").unwrap();
    let m = re.find("line one\nline 2\n").unwrap();
    assert_eq!(m.as_str(), "line 2");

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
        static ref RE_REMOVE_SHORT_QUOTES: Regex = Regex::new("(?P<a>......).......(?P<b>.*)").unwrap();
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
        static ref RE_REMOVE_SHORT_QUOTES: Regex = Regex::new(r#"(?P<yes1>\^.*)(?P<no>,".*",)(?P<yes2>.*$)"#).unwrap();
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    capture_across_mult_lines(loaded_string1.clone());
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    match res {
        Err(ee) => panic!("remove_quoted_fields::RegEx Error: {ee}\n"),
        Ok(ss)  => ss
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ replace aaa with a1a
fn repl_aaa_w_a1a(ss: String) -> String {

    lazy_static! {
        static ref RE_REPL_AAA_W_A1A: Regex = Regex::new(
    }

}
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    fs::write("/usr/local/sys/rust/data/q1_p_data.csv", ss).expect("check::Write Error");
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// use unicode_segmentation::UnicodeSegmentation;
// use lazy_static::lazy_static;
// use std::io::prelude::Path;
// use std::fmt;
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

    let ss = read_file("q0_data.csv");      // wherever you are when you run   ∞ cargo run
    print!("File Content: \n{}\n", ss);

    let ss = read_file("../q1_data.csv");   // parent dir of curr clocation when you run   ∞ cargo run
    print!("File Content: \n{}\n", ss);
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

data_out.txt
j.large.p
j.small.p
q1_p_data.csv
q2_p_data.csv
q3_p_data.csv
x0_data.csv
x1_data.csv

vim q01_p_data.csv  && vim q02_p_data.csv && vim q03_p_data.csv
vim q11_p_data.csv  && vim q12_p_data.csv && vim q13_p_data.csv

vim q12_p_data.csv && vim q13_p_data.csv && vim q14_p_data.csv && vim q15_p_data.csv

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•

•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
․5․λ Code Pit Fragments  



•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•



•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
․1․ End of (Rev․ Chrono)
․0․ END OF SHORT ARTICLES (Rev․ Chrono)
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·







                           ·════════════════════════════··════════════════════════════··════════════════════════════·
                           ·════════════ 🔻 ════════════··════════════ 🔻 ════════════··════════════ 🔻 ════════════·
                           ·════════════════════════════··════════════════════════════··════════════════════════════·



                                         💥                            💥                            💥
                                       💥💥💥                        💥💥💥                        💥💥💥
                                         💥                            💥                            💥





                                                         ·════════════════════════════·
                                                         ·════════════ 🔻 ════════════·
                                                         ·════════════════════════════·



                                                                      💥💥
                                                                     💥  💥
                                                                      💥💥



                                                         ·════════════════════════════·
                                                         ·════════════ 🔻 ════════════·
                                                         ·════════════════════════════·





                                         💥                            💥                            💥
                                       💥💥💥                        💥💥💥                        💥💥💥
                                         💥                            💥                            💥



                           ·════════════════════════════··════════════════════════════··════════════════════════════·
                           ·════════════ 🔻 ════════════··════════════ 🔻 ════════════··════════════ 🔻 ════════════·
                           ·════════════════════════════··════════════════════════════··════════════════════════════·







·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․9․⛩κ✨λ ARCHIVE OF OLDER COMPLETE MODULES (Fwd․ Chrono)

․4․ PREAMBLE:

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
❓𐤒 QaaS ─ Questions Answers Actions § Searches

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
🔭 URLs →

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
⭐ν NOTES:


•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·

                                                                 ·════ 🔻 ════·
                                                                       💥
                                                                 ·════ 🔻 ════·

·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․7․✨λ  (Fwd․ Chrono)

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ s0_scratchpad  ι✧21․11․22✦10․08․26․ 🌎η ✧22․10․22․
#![allow(dead_code)]
extern crate regex;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use lazy_static::lazy_static;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`
#[cfg(test)]
mod test_regex {
    use super::*;

    const TEST_STR: &str = r#"\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln02: aa,   ddd1,'  bb,   ccc,   ddd2',   eee,   f,  \n\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln02: aa,   ddd1,'  bb,   ccc,   ddd2',   eee,   f,  \n\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln02: aa,   ddd1,'  bb,   ccc,   ddd2',   eee,   f,  \n\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln02: aa,   ddd1,'  bb,   ccc,   ddd2',   eee,   f,  \n\
ln03: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln04: aa,   ddd3,   bb,   ccc,   ddd44,   eee,"  f,  \n\
ln05: a",   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln06: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln07: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln08: aa,   ddd3,   bb,   ccc,   ddd44,   eee,"  f,  \n\
ln09: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln10: a",   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln11: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln12: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln13: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln02: aa,   ddd1,'  bb,   ccc,   ddd2',   eee,   f,  \n\
ln01: aa,   ddd1,"  bb,   ccc,   ddd2",   ee',   f,  \n\
ln14: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln15: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln16: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
ln17: aa,   ddd3,   bb,   ccc,   ddd44,   eee,   f,  \n\
Downloaded Data (x. Prefix)                          \n\
Controlled Text for Testing RegEx Captures
"#;

    #[test]
    fn test_preamble() {
        print!("TEST_STR: {TEST_STR} \n")
    }

}

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ common functions
fn prt_chars(ss: &String) {
    print!("    ss.chars(): ");
    for val in ss.chars() {
        print!("{val} ");
    }
    print!("\n\n");
}


///λ Read a file the old way
fn read_file(file_path: &str) -> String {

    let path = Path::new(file_path);
    print!("path.display(): {}\n", path.display());

    let mut ff = match File::open(&path) {
        Err(ee) => panic!("Open Error: {ee}\n"),
        Ok(ff)  => ff,
    };


    let mut ss = String::new();
    match ff.read_to_string(&mut ss) {
        Err(ee) => panic!("read_file::Read Error: {ee}\n"),
        Ok(_) => ss
    }
}


///λ Capture across multiple lines
fn capture_across_mult_lines(ss: String) {

    //  •═══··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    print!("{}🎡𐡋 Checking: Capturing across multiple lines \n", C_LL);

    // first a simple patern
    let re = Regex::new(r#"(dd1)"#).unwrap();

    for cap in re.captures_iter(&ss) {
        print!("\nMatch:   {:?}\n", cap);
    }

    print!("\n🎡𐡋 Capture all text inside: (,' text ',)  \n");

    // ignoring all delimiters between single quotes in one line (,'  text  ',)
    let re = Regex::new(r#",'(.*)',"#).unwrap();

    for cap in re.captures_iter(&ss) {
        print!("\nMatch:   {:?}\n", cap);
    }

    print!("\n🎡𐡋 Capture all text inside: (,\" text \",)  \n");

    // ignoring all delimiters between double quotes in multiple lines (,"  text  ",)
    let re = Regex::new(r#"(?s),"(.*)","#).unwrap();

    for cap in re.captures_iter(&ss) {
        print!("\n\nMatch:   {:?}\n\n", cap);
    }
}


///λ this RegEx Helper looseley ratches an IPv4 addr
fn has_ipv4(ss: &str) -> bool {

    lazy_static! {
        static ref RE_IPV4: Regex = Regex::new(r" \d\d?\d?\.\d\d?\d?\.\d\d?\d?\.\d\d?\d? ").unwrap();
    }
    RE_IPV4.is_match(ss)
}


///λ This RegEx Helper removes (," ... ",) in a single line
fn remove_1_inline_quotes(ss: String) -> String {

    lazy_static! {
        static ref RE_REMOVE_1_INLINE_QUOTES: Regex = Regex::new(r#"(?P<yes1>.*)(?P<no>,".*",)(?P<yes2>.*)"#).unwrap();
    }
    let res = RE_REMOVE_1_INLINE_QUOTES.replace_all(&ss, "≺1:$yes1≻,--removed_inline_quotation--,≺2:$yes2≻");
    res.to_string()
}


///λ This RegEx Helper removes (," ... ",) across multiple lines - this one matches \n in entire regex
fn remove_2_multiline_quotes(ss: String) -> String {

    lazy_static! {                                                          //    `(?m)` = multi-line mode
        static ref RE_REMOVE_2_MULTILINE_QUOTES: Regex = Regex::new(r#"((?ms:.)(?P<yes1>.*)(?P<no>,".*",)(?P<yes2>.*))"#).unwrap();
    }
    let res = RE_REMOVE_2_MULTILINE_QUOTES.replace_all(&ss, "≺1:$yes1≻,--removed_multiline_quotation--,≺2:$yes2≻");
    res.to_string()
}


///λ This RegEx Helper removes (," ... ",) across multiple lines - this one matches \n in entire regex
fn remove_3_multiline_quotes(ss: String) -> String {

    lazy_static! {                                                          //    `(?m)` = multi-line mode
        static ref RE_REMOVE_3_MULTILINE_QUOTES: Regex = Regex::new(r#"((?P<yes1>(?ms:.).*)(?P<no>,".*",)(?P<yes2>.*))"#).unwrap();
    }
    let res = RE_REMOVE_3_MULTILINE_QUOTES.replace_all(&ss, "≺1:$yes1≻,--removed_multiline_quotation--,≺2:$yes2≻");
    res.to_string()
}


///⭐νλ This RegEx Helper removes (," ... ",) across multiple lines - this one matches \n in entire regex
fn remove_4_multiline_quotes(ss: String) -> String {

    lazy_static! {                                                          //    `(?m)` = multi-line mode
        static ref RE_REMOVE_4_MULTILINE_QUOTES: Regex = Regex::new(r#"((?P<yes1>.*)(?P<no>(?ms:,".*?",))(?P<yes2>.*))"#).unwrap();
    }
    let res = RE_REMOVE_4_MULTILINE_QUOTES.replace_all(&ss, "≺1:$yes1≻,--removed_quoted_field--,≺2:$yes2≻");
    res.to_string()
}


///λ This RegEx Helper removes (," ... ",) across multiple lines - this one matches \n in entire regex
fn remove_5_multiline_quotes(ss: String) -> String {

    lazy_static! {                                                          //    `(?m)` = multi-line mode
        static ref RE_REMOVE_5_MULTILINE_QUOTES: Regex = Regex::new(r#"((?P<yes1>.*)(?P<no>,".*",)(?P<yes2>(?ms:.).*))"#).unwrap();
    }
    let res = RE_REMOVE_5_MULTILINE_QUOTES.replace_all(&ss, "≺1:$yes1≻,--removed_multiline_quotation--,≺2:$yes2≻");
    res.to_string()
}


///λ This RegEx Helper removes (," ... ",) across multiple lines - this one matches \n in entire regex
fn remove_7_multiline_quotes(ss: String) -> String {

    lazy_static! {                                                          //    `(?m)` = multi-line mode
        static ref RE_REMOVE_7_MULTILINE_QUOTES: Regex = Regex::new(r#"((?m)(?s:.)(?P<yes1>(?m).*)(?P<no>(?m),".*",)(?P<yes2>(?m).*))"#).unwrap();
    }
    let res = RE_REMOVE_7_MULTILINE_QUOTES.replace_all(&ss, "≺1:$yes1≻,--removed_multiline_quotation--,≺2:$yes2≻");
    res.to_string()
}


///λ This RegEx Helper removes (," ... ",) across multiple lines - this one matches \n only in <no>
fn remove_8_multiline_quotes(ss: String) -> String {

    lazy_static! {                                                          //    `(?m)` = multi-line mode
        static ref RE_REMOVE_8_MULTILINE_QUOTES: Regex = Regex::new(r#"((?m)(?P<yes1>(?m).*)(?s:.)(?P<no>(?m),".*",)(?P<yes2>(?m).*))"#).unwrap();
    }
    let res = RE_REMOVE_8_MULTILINE_QUOTES.replace_all(&ss, "≺1:$yes1≻,--removed_multiline_quotation--,≺2:$yes2≻");
    res.to_string()
}


///λ check_regex_helpers checks functionality in the in the development vector
pub fn check_regex_helpers() {
    // print!("{}🎡𐡋 checking regex helpers  \n\n", C_LL);

    let loaded_string1 = fs::read_to_string("/usr/local/sys/rust/data/x1_data.csv").expect("check::Read Error");
    let parsed_string = remove_1_inline_quotes(loaded_string1.clone());
    fs::write("/usr/local/sys/rust/data/q11_p_data.csv", &parsed_string).expect("check::Write Parse1 Error");

    let parsed_string = remove_2_multiline_quotes(loaded_string1.clone());
    fs::write("/usr/local/sys/rust/data/q12_p_data.csv", &parsed_string).expect("check::Write Parse2 Error");

    let parsed_string = remove_3_multiline_quotes(loaded_string1.clone());
    fs::write("/usr/local/sys/rust/data/q13_p_data.csv", &parsed_string).expect("check::Write Parse3 Error");

    let parsed_string = remove_4_multiline_quotes(loaded_string1.clone());
    fs::write("/usr/local/sys/rust/data/q14_p_data.csv", &parsed_string).expect("check::Write Parse4 Error");

    let parsed_string = remove_5_multiline_quotes(loaded_string1);
    fs::write("/usr/local/sys/rust/data/q15_p_data.csv", &parsed_string).expect("check::Write Parse5 Error");

    let re = Regex::new(r"(?m)^line \d+").unwrap();
    let m = re.find("line one\nline 2\n").unwrap();
    assert_eq!(m.as_str(), "line 2");

    let re = Regex::new(r"(?m)^line \d+").unwrap();
    let m = re.find("line one\nline 2\n").unwrap();
    assert_eq!(m.as_str(), "line 2");
}



// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// Constants Types § Enums

const C_LL: &str = "\n•═══════════··══════════════════·═══════════════════··═══════════•\n";

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ check is the integration test executive function for this module
pub fn check() -> Result<(), Box<dyn Error>> {
    // print!("{}🎡𐡋 Checking: s1_exec  \n\n", C_LL);

    check_regex_helpers();

    Ok(())
    // panic!("for: No Reason");
}

•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
․5․ 


/// The Code Pit
/ *

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
FROM LEXER
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
enum Lexium {
    Cm,
    Dot,
    Els,
    End,
    Eof,
    Er,
    Fld,
    Id,
    Iff,
    Lb,
    Lbb,
    Ls,
    Lss,
    Nl,
    Num,
    Pip,
    Rb,
    Rbb,
    Rng,
    Rs,
    Rss,
    Sp,
    Stg,
    st,
    Txt,
}

impl Lexium {
    fn new(&self) -> String {
        match self {
            Self::Cm    => ",".to_string(),
            Self::Dot   => ".".to_string(),
            Self::Els   => "".to_string(),
            Self::End   => "".to_string(),
            Self::Eof   => "".to_string(),
            Self::Er    => "".to_string(),
            Self::Fld   => "".to_string(),
            Self::Id    => "".to_string(),
            Self::Iff   => "".to_string(),
            Self::Lb    => "[".to_string(),
            Self::Lbb   => "[[".to_string(),
            Self::Ls    => "{ ".to_string(),
            Self::Lss   => "{{".to_string(),
            Self::Nl    => "\n".to_string(),
            Self::Num   => "".to_string(),
            Self::Pip   => "|".to_string(),
            Self::Rb    => "]".to_string(),
            Self::Rbb   => "]]".to_string(),
            Self::Rng   => "".to_string(),
            Self::Rs    => " }".to_string(),
            Self::Rss   => "}}".to_string(),
            Self::Sp    => " ".to_string(),
            Self::Stg   => "".to_string(),
            Self::St    => "".to_string(),
            Self::Txt   => "".to_string(),
        }
    }
}

struct Lex {
    cm:   crate::sysops::Lexium::Cm,
    dot:  crate::sysops::Lexium::Dot,
    els:  crate::sysops::Lexium::Els,
    end:  crate::sysops::Lexium::End,
    eof:  crate::sysops::Lexium::Eof,
    er:   crate::sysops::Lexium::Er,
    fld:  crate::sysops::Lexium::Fld,
    id:   crate::sysops::Lexium::Id,
    iff:  crate::sysops::Lexium::Iff,
    lb:   crate::sysops::Lexium::Lb,
    lbb:  crate::sysops::Lexium::Lbb,
    ls:   crate::sysops::Lexium::Ls,
    lss:  crate::sysops::Lexium::Lss,
    nl:   crate::sysops::Lexium::Nl,
    num:  crate::sysops::Lexium::Num,
    pip:  crate::sysops::Lexium::Pip,
    rb:   crate::sysops::Lexium::Rb ,
    rbb:  crate::sysops::Lexium::Rbb,
    rng:  crate::sysops::Lexium::Rng,
    rs:   crate::sysops::Lexium::Rs,
    rss:  crate::sysops::Lexium::Rss,
    sp:   crate::sysops::Lexium::Sp,
    stg:  crate::sysops::Lexium::Stg,
    st:   crate::sysops::Lexium::St,
    txt:  crate::sysops::Lexium::Txt,
}



struct Lex {
    cm:   Lexium::Cm,
    dot:  Lexium::Dot,
    els:  Lexium::Els,
    end:  Lexium::End,
    eof:  Lexium::Eof,
    er:   Lexium::Er,
    fld:  Lexium::Fld,
    id:   Lexium::Id,
    iff:  Lexium::Iff,
    lb:   Lexium::Lb,
    lbb:  Lexium::Lbb,
    ls:   Lexium::Ls,
    lss:  Lexium::Lss,
    nl:   Lexium::Nl,
    num:  Lexium::Num,
    pip:  Lexium::Pip,
    rb:   Lexium::Rb ,
    rbb:  Lexium::Rbb,
    rng:  Lexium::Rng,
    rs:   Lexium::Rs,
    rss:  Lexium::Rss,
    sp:   Lexium::Sp,
    stg:  Lexium::Stg,
    st:   Lexium::St,
    txt:  Lexium::Txt,
}

struct Lex {
    cm:   Cm,
    dot:  Dot,
    els:  Els,
    end:  End,
    eof:  Eof,
    er:   Er,
    fld:  Fld,
    id:   Id,
    iff:  Iff,
    lb:   Lb,
    lbb:  Lbb,
    ls:   Ls,
    lss:  Lss,
    nl:   Nl,
    num:  Num,
    pip:  Pip,
    rb:   Rb ,
    rbb:  Rbb,
    rng:  Rng,
    rs:   Rs,
    rss:  Rss,
    sp:   Sp,
    stg:  Stg,
    st:   St,
    txt:  Txt,
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
enum Lex {
    Cm(String),
    Dot(String),
    Els(String),
    End(String),
    Eof(String),
    Er(String),
    Fld(String),
    Id(String),
    Iff(String),
    Lb(String),
    Lbb(String),
    Ls(String),
    Lss(String),
    Nl(String),
    Num(String),
    Pip(String),
    Rb(String),
    Rbb(String),
    Rng(String),
    Rs(String),
    Rss(String),
    Sp(String),
    Stg(String),
    St(String),
    Txt(String),
}

impl Lex {
    fn get(&self) -> String {
        match self {
            Self::Cm(String)    => ",".to_string(),
            Self::Dot(String)   => ".".to_string(),
            Self::Els(String)   => "".to_string(),
            Self::End(String)   => "".to_string(),
            Self::Eof(String)   => "".to_string(),
            Self::Er(String)    => "".to_string(),
            Self::Fld(String)   => "".to_string(),
            Self::Id(String)    => "".to_string(),
            Self::Iff(String)   => "".to_string(),
            Self::Lb(String)    => "[".to_string(),
            Self::Lbb(String)   => "[[".to_string(),
            Self::Ls(String)    => "{ ".to_string(),
            Self::Lss(String)   => "{{".to_string(),
            Self::Nl(String)    => "\n".to_string(),
            Self::Num(String)   => "".to_string(),
            Self::Pip(String)   => "|".to_string(),
            Self::Rb(String)    => "]".to_string(),
            Self::Rbb(String)   => "]]".to_string(),
            Self::Rng(String)   => "".to_string(),
            Self::Rs(String)    => " }".to_string(),
            Self::Rss(String)   => "}}".to_string(),
            Self::Sp(String)    => " ".to_string(),
            Self::Stg(String)   => "".to_string(),
            Self::St(String)    => "".to_string(),
            Self::Txt(String)   => "".to_string(),
        }
    }
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
enum Lex {
    Cm(String),
    Dot(String),
    Els(String),
    End(String),
    Eof(String),
    Er(String),
    Fld(String),
    Id(String),
    Iff(String),
    Lb(String),
    Lbb(String),
    Ls(String),
    Lss(String),
    Nl(String),
    Num(String),
    Pip(String),
    Rb(String),
    Rbb(String),
    Rng(String),
    Rs(String),
    Rss(String),
    Sp(String),
    Stg(String),
    St(String),
    Txt(String),
}

impl Lex {
    fn get(&self) -> String {
        match self {
            Self::Cm    => ",".to_string(),
            Self::Dot   => ".".to_string(),
            Self::Els   => "".to_string(),
            Self::End   => "".to_string(),
            Self::Eof   => "".to_string(),
            Self::Er    => "".to_string(),
            Self::Fld   => "".to_string(),
            Self::Id    => "".to_string(),
            Self::Iff   => "".to_string(),
            Self::Lb    => "[".to_string(),
            Self::Lbb   => "[[".to_string(),
            Self::Ls    => "{ ".to_string(),
            Self::Lss   => "{{".to_string(),
            Self::Nl    => "\n".to_string(),
            Self::Num   => "".to_string(),
            Self::Pip   => "|".to_string(),
            Self::Rb    => "]".to_string(),
            Self::Rbb   => "]]".to_string(),
            Self::Rng   => "".to_string(),
            Self::Rs    => " }".to_string(),
            Self::Rss   => "}}".to_string(),
            Self::Sp    => " ".to_string(),
            Self::Stg   => "".to_string(),
            Self::St   => "".to_string(),
            Self::Txt   => "".to_string(),
        }
    }

}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
  № Cm:   ",".to_string(),  ‡ comma
  № Dot:  ".".to_string(),  ‡ dot
  № Els:  "".to_string(),   ‡ else
  № End:  "".to_string(),   ‡ end
  № Eof:  "".to_string(),   ‡ eof
  № Er:   "".to_string(),   ‡ Er
  № Fld:  "".to_string(),   ‡ field
  № Id:   "".to_string(),   ‡ id
  № Iff:   "".to_string(),  ‡ Iff
  № Lb:   "[".to_string(),  ‡ left bracket
  № Lbb:  "[[".to_string(), ‡ double left bracket
  № Ls:   "{ ".to_string(), ‡ left set
  № Lss:  "{{".to_string(), ‡ double left set
  № Nl:   "\n".to_string(), ‡ new line
  № Num:  "".to_string(),   ‡ number
  № Pip:  "|".to_string(),  ‡ pipe
  № Rb:   "]".to_string(),  ‡ right bracket
  № Rbb:  "]]".to_string(), ‡ double right bracket
  № Rng:  "".to_string(),   ‡ range
  № Rs:   " }".to_string(), ‡ right set
  № Rss:  "}}".to_string(), ‡ double right set
  № Sp:   " ".to_string(),  ‡ space
  № Stg:  "".to_string(),   ‡ string
  № St:  "".to_string(),    ‡ string literal
  № Txt:  "".to_string(),   ‡ raw text


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    errorLexium     lexiumNameT = iota      // when error happens lexiumValue becomes the text of the error
    dotLexium                               // '.'
    eofLexium                               // value is text of error

    ifLexium                                // keywords: 'if', 'else' and 'end'
    elseLexium
    endLexium

    fieldLexium                             // starting with '.'
    idLexium                                // identifier

    leftMetaLexium                          // '{{'
    rghtMetaLexium                          // '}}'

    numberLexium                            // a number eg: '123.45'
    pipeLexium                              // '|'
    rangeLexium                             // keyword: 'range'

    rawStringLexium                         // raw ('') quoted string, eg: 'aaa bbb ccc ddd'
    stringLexium                            // quoted ("") string, eg: "aaa bbb ccc"
    textLexium                              // plain text
)

const (
    errorC  = "";       ifC     = "";
    dotC    = ".";      elseC   = "";
    eofC    = "";       endC    = "";

    fieldC  = "";       leftMetaC   = "{{";
    iDC     = "";       rightMetaC  = "}}";

    numberC = "";       rawStringC  = "";
    pipeC   = "|";      stringC     = "";
    rangeC  = "";       textC       = "";

    emptyRuneC  rune = 0;
    eofIntC     int = 0;
)




•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
․1․ End of
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·


                                             ·══════ 💥 ══════··══════ 🔻 ══════··══════ 💥 ══════·



                                                               ·══════ 🔻 ══════·


                                                                       💥


                                                               ·══════ 🔻 ══════·



                                             ·══════ 💥 ══════··══════ 🔻 ══════··══════ 💥 ══════·



·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․7․✨λ  (Fwd․ Chrono)




•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
․1․ End of
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·


                                             ·══════ 💥 ══════··══════ 🔻 ══════··══════ 💥 ══════·



                                                               ·══════ 🔻 ══════·


                                                                       💥


                                                               ·══════ 🔻 ══════·



                                             ·══════ 💥 ══════··══════ 🔻 ══════··══════ 💥 ══════·



·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․7․✨λ  (Fwd․ Chrono)




•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
․1․ End of
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·


                                             ·══════ 💥 ══════··══════ 🔻 ══════··══════ 💥 ══════·



                                                               ·══════ 🔻 ══════·


                                                                       💥


                                                               ·══════ 🔻 ══════·



                                             ·══════ 💥 ══════··══════ 🔻 ══════··══════ 💥 ══════·



·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․7․✨λ  (Fwd․ Chrono)




•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
․1․ End of
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·


                                             ·══════ 💥 ══════··══════ 🔻 ══════··══════ 💥 ══════·



                                                               ·══════ 🔻 ══════·


                                                                       💥


                                                               ·══════ 🔻 ══════·



                                             ·══════ 💥 ══════··══════ 🔻 ══════··══════ 💥 ══════·



·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․7․✨λ  (Fwd․ Chrono)


•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
․1․ End of
․0․ END OF ARCHIVE OF OLDER COMPLETE MODULES (FWD․ Chrono)
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·


                                             ·══════ 💥 ══════··══════ 🔻 ══════··══════ 💥 ══════·



                                                               ·══════ 🔻 ══════·


                                                                       💥


                                                               ·══════ 🔻 ══════·



                                             ·══════ 💥 ══════··══════ 🔻 ══════··══════ 💥 ══════·


·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․4․🦢ζι✧©1991․05․01․TMP: Nomenclature § Glossary․


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
․3․🏛№ Header Legend:

ᚖ․this․File․Classification    Φ․this.File─Name     ᚖ․Cat1․Primary Category           ᚖ․Subj1․Primary Subjet Areas
ο․this․File․Owner─Name                             ᚖ․Cat2․Secondary Category         ᚖ․Subj2․Secondary or Specific Subjet Areas

ᚖ․File․Classification:        • Γ0․ Pub․p;  Γ1․ Int․s;    Γ2․ Conf․s;   Γ3․ Secr․s;
                              • Γ4․ TS․s;   Γ5․ Comp1․s;  Γ6․ Comp2․s;  Γ[789]․ Crypto․k;


•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •                          ·                   ·                   ·                   ·            •
•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •                          ·                   ·                   ·                   ·            •
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·

                                                                 ·════ 🔻 ════·

                                                                       💥

                                                                 ·════ 🔻 ════·

·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··════════════════ 🔻 ══════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
․4․🦢ζ TMP: Staging § Footer


•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
•           ·                   ·                   ·                   ·                   ·                   ·                   ·            •
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
·═══════════··══════════════════════════════════════··══════════════════════════════════════··══════════════════════════════════════··═══════════·
─ → ⇒ ► 📎 🔑 ⭐ 💥 ⚡ 🈳 🈴 ⛩ 📐 🎡 ⏰ 🏁 ✅ 🏃 🦎 ⭕️ 🚫 👥 🖖 🖖 👍 👎 ⮕ ⬅ ⬆ ⬇          ․Jan․F․M․․Apr․M․J․․Jul․A․S․․Oct․N․D․
․4․©1991․𐍂․𐡇․𐒱lpíz𐓙r․(Rafael H. Alpizar ι․1991․05․03․) § ® ● • · 、 〞 ∞ ∅ ✝ ✔ ✘ ≙ ᚖ ⋯ ⋱ ⋰ ․A․․․B․C․․D․․․E․F․․G․․․H․I․․J․․․K․L․
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•


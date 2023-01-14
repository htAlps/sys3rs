// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// ✨λ sysops::mod  ι✧21․12․25✦16․50․24․  🌎η ✧22․12․29․✧22․11․12․✧22․08․22․✧22․08․19․✧22․08․16․✧22․08․07․✧22․08․05․✧22․07․04․✧22․06․22․
pub mod s4_metrics;
use lib3::q0_env::{get_cmd_code};
// use lib3;

const _USAGE_TABLE: &str = r#"👎ς ERROR: Invalid Command.

 ╔══════════════════════════════════════════════════════════════════════════════════════════╗ 
 ║ β Usage: enter a valid code from this table (codes are lower-case)                       ║ 
 ╠══════╤═════════════════════╤═════════════════════════════════════════════════════════════╣ 
 ║ Code │ Mod/Obj─Type/Func   │ Operation: Description                                      ║ 
 ╠══════╪═════════════════════╪═════════════════════════════════════════════════════════════╣ 
 ║ 2hs+ │ 2/HashSet/`+`       │ Add HashSets:     y2hs.csv <- x2hs1.csv + x2hs2.csv         ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ 2hs- │ 2/HashSet/`-`       │ Subtr HashSets:   y2hs.csv <- x2hs1.csv - x2hs2.csv         ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ 2hm+ │ 2/HashMap/`+`       │ Add HashMaps:     y2hm.csv <- x2hm1.csv + x2hm2.csv         ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ 2hm- │ 2/HashMap/`-`       │ Subtr HashMaps:   y2hm.csv <- x2hm1.csv - x2hm2.csv         ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ 3rmq │ 3/CSV/Clean         │ Clean CSV:        y3clean.csv <- clean(x3dirty.csv)         ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ 4met │ 4/CSV/Metrics       │ Get Fold Metrics: y4metrics_fold1/2.csv <- fold(x4raw.csv)  ║ 
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ chki │ All/Int─Tst/Check   │ Run INT-Tests: recurse through check() fns across all Mods  ║
 ╟──────┼─────────────────────┼─────────────────────────────────────────────────────────────╢ 
 ║ help │ N/A                 │ Print more detail help on the system                        ║
 ╚══════╧═════════════════════╧═════════════════════════════════════════════════════════════╝ 
"#;


const _HELP_TABLE: &str = r#"
 ╔══════════════════════════════════════════════════════════════════════════════════════════════════╗ 
 ║ β HELP:  Below is the list of functions available and a quick description                        ║ 
 ╠══════╤═════════════════════════════════════════════╤═════════════════════════════════════════════╣ 
 ║ Code │           Operation                         │     Description                             ║ 
 ╠══════╪═════════════════════════════════════════════╪═════════════════════════════════════════════╣ 
 ║ 2hs+ │ y2hs.csv <- x2hs1.csv + x2hs2.csv           │ Add 2 HashSets                              ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ 2hs- │ y2hs.csv <- x2hs1.csv - x2hs2.csv           │ Subtract 2 HashSets                         ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ 2hm+ │ y2hm.csv <- x2hm1.csv + x2hm2.csv           │ Add 2 HashMaps                              ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ 2hm- │ y2hm.csv <- x2hm1.csv - x2hm2.csv           │ Subtract 2 HashMaps                         ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ 3rmq │ y3clean.csv <- clean(x3dirty.csv)           │ Clean Multiline Fields and Other Stuff      ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ 4met │ y4metrics_fold[1/2].csv <- fold(x4raw.csv)  │ Generate Metrics on a Foldable CSV          ║ 
 ╟──────┼─────────────────────────────────────────────┼─────────────────────────────────────────────╢ 
 ║ chki │ recurse through check() functions           │ Runs INT Testing of All Mods and Libs       ║
 ╚══════╧═════════════════════════════════════════════╧═════════════════════════════════════════════╝ 
"#;

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
//λ unit tests -- REALLY HARD TO TEST WITHOUT `use super::*;`

// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•


/// check int-tests the active system as a whole
pub fn check() -> Result<(), String> {

    let my_location = "sysops::run"; 
    print!("\n🎡𐡋 running: {}\n", my_location);
    let cmd_code = get_cmd_code();
    
    let my_location = "s1_exec::run";
    match cmd_code {
        "2hs+"  => {                // y2hs.csv <- x2hs1.csv + x2hs2.csv  
            print!("\n🎡𐡋 {my_location}::2hs+ \n");
        }
        "2hs-"  => {                // y2hs.csv <- x2hs1.csv - x2hs2.csv  
            print!("\n🎡𐡋 {my_location}::2hs- \n");
        }
        "2hm+"  => {                // y2hm.csv <- x2hm1.csv + x2hm2.csv  
            print!("\n🎡𐡋 {my_location}::2hm+ \n");
        }
        "2hm-"  => {                // y2hm.csv <- x2hm1.csv - x2hm2.csv  
            print!("\n🎡𐡋 {my_location}::2hm- \n");
        }
        "3rmq"  => {                // y3clean.csv <- clean(x3dirty.csv)  
            print!("\n🎡𐡋 {my_location}::3rmq \n");
        }
        "4met"  => {                // y4metrics_fold1/2.csv <- fold(x4raw.csv)  ║ 
            print!("\n🎡𐡋 {my_location}::4met \n");
        }
        "chki"  => {                // Run INT-Tests 
            print!("\n🎡𐡋 {my_location}::chkichki \n");
        }
        "help"  => {                // Help
            print!("\n🎡𐡋 {my_location}::help \n");
        }
        _ => {
            Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        }
}



// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ run() is the system's exec fn for sysops module; 
pub fn run() -> Result<(), String> {

    match s1_metrics::run() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}


//λ The Code Pit
/*
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ run() gets the function-code to be executed and runs the corresponding fn
pub fn run() -> Result<(), String> {

    let my_location = "sysops::run"; 
    print!("\n🎡𐡋 running: {}\n", my_location);
    let cmd_code = get_cmd_code();
    
    let my_location = "s1_exec::run";
    match cmd_code {
        "2hs+"  => {                // y2hs.csv <- x2hs1.csv + x2hs2.csv  
            match s2_hash::add_hashsets() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => Ok(()),
            }
        }
        "2hs-"  => {                // y2hs.csv <- x2hs1.csv - x2hs2.csv  
            match s2_hash::sub_hashsets() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => Ok(()),
            }
        }
        "2hm+"  => {                // y2hm.csv <- x2hm1.csv + x2hm2.csv  
            match s2_hash::add_hashmaps() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => Ok(()),
            }
        }
        "2hm-"  => {                // y2hm.csv <- x2hm1.csv - x2hm2.csv  
            match s2_hash::sub_hashmaps() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => Ok(()),
            }
        }
        "3rmq"  => {                // y3clean.csv <- clean(x3dirty.csv)  
            match s3_regex::clean_csv() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => Ok(()),
            }
        }
        "4met"  => {                // y4metrics_fold1/2.csv <- fold(x4raw.csv)  ║ 
            match s4_metrics::gen_folds() {
                Err(ee) => Err(format!("{ee}⟸ {my_location}")),
                _ => Ok(()),
            }
        }
        "chki"  => {                // Run INT-Tests 
            print!("\n🎡𐡋 {my_location}::chki \n");
        }
        "help"  => {                // Help
        }
        _ => {
            Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        }
}

•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
/// check int-tests the active system as a whole
pub fn check() -> Result<(), String> {

    let my_location = "sysops::check";
    match s1_metrics::check() {                                     // checking metrics calculations 
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
    match s1_metrics::run() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
// •════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
///λ run() is the system's exec fn for sysopss module; 
pub fn run() -> Result<(), String> {

    let my_location = "s1_exec::run";
    let lex1 = Lex::new();
    print!("lex1: \n{lex1}");
    
    match map_iter_2() {
        Err(ee) => Err(format!("{ee}⟸ {my_location}")),
        _ => Ok(()),
    }
}


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
λ Crates § Modules

mod sysops_exec;
mod a84_re_multiline;               use a84_re_multiline::{check};
mod a83_regex_basics;               use a83_regex_basics::{check};
mod a82_string_methods;             use a82_string_methods::{check};


•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
mod sysops_exec;                    use sysops_exec::{check};
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
•═══════════··══════════════════·═══════════════════··══════════════════·═══════════════════··═══════════•
*/
// End Of The Code Pit


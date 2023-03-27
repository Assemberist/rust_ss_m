pub mod statem;

fn main() {
    let mut mach: [statem::Machine; 4] = statem::init_statems();
    {
        println!("init:sas -> {}", statem::query(String::from("init:sas"), &mut mach));
        println!("run:sas -> {}", statem::query(String::from("run:sas"), &mut mach));
        println!("list ->\n{}", statem::query(String::from("list:"), &mut mach));
    }
}

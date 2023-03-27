pub mod statem;

fn main() {
    let mut mach: [statem::Machine; 4] = statem::init_statems();
    {
        println!("init:sas -> {}", statem::query(String::from("init:sas"), &mut mach));
        println!("run:sas -> {}", statem::query(String::from("run:sas"), &mut mach));
    }
    mach.map(|m| println!("{}\n{}", m.name, statem::state_to_string(m.state)));
}

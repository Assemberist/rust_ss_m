#[derive(PartialEq)]
pub enum StateT{
    INIT,
    RUN,
    PAUSE,
    CLOSE
}

pub fn state_to_string(state: StateT) -> String {
    String::from(
        match state {
            StateT::INIT => "Init",
            StateT::RUN => "Run",
            StateT::PAUSE => "Pause",
            StateT::CLOSE => "Close"
        }
    )
}

pub struct Machine{
    pub state:  StateT,
    pub name:   String
}

impl Machine{
    fn new(_i: u32) -> Self {
        Machine {
            state: StateT::CLOSE,
            name: String::new(),
        }
    }
}

pub fn init_statems() -> [Machine; 4] {
    [0; 4].map(|i| Machine::new(i))
}

pub fn query(buffer: String, machines: &mut [Machine; 4]) -> String {
    let (head, tail) = buffer.as_str().split_once(':').expect("");

    let handler = 
            match head {
                "init" => hand_init,
                "run" => hand_run,
                "pause" => hand_pause,
                "stop" => hand_stop,
                "enough" => hand_enough,
                "exterminantus" => hand_exterminantus,
/*                "get" => hand_get,
                "exit" => hand_exit_server,
                "list" => hand_list,
   */             _other => hand_strange
            };

    handler(tail, machines)
}

fn find_machine(name: &str, machines: &mut [Machine; 4]) -> (bool, usize) {
    let mut i = 0;
    while i < 4 {
        if machines[i].name == name { 
            return (true, i);
        }
        i = i+1;
    }

    (false, 0)
}

fn hand_init(args: &str, machines: &mut [Machine; 4]) -> String {
    if args == "" {
        return String::from("Error: No name");
    }

    let answer =
        match find_machine(args, machines) {
            (false, _) =>
                match find_machine("", machines) {
                    (false, _) => "Error: No free machine",

                    (true, i) => {
                        machines[i].name = String::from(args);
                        machines[i].state = StateT::INIT;
                        "Ok"
                    }
                },

            (true, _) => "Error: Already created"
        };

    String::from(answer)
}

fn hand_run(args: &str, machines: &mut [Machine; 4]) -> String {
    if args == "" {
        return String::from("Error: No name");
    }

    let answer =
        match find_machine(args, machines) {
            (false, _) => "Error: Not found",
            (true, i) =>
                if machines[i].state == StateT::RUN { "Error: Already running" }
                else { machines[i].state = StateT::RUN; "Ok" }
        };

    String::from(answer)
}

fn hand_pause(args: &str, machines: &mut [Machine; 4]) -> String {
    if args == "" {
        return String::from("Error: No name");
    }

    let answer =
        match find_machine(args, machines) {
            (false, _) => "Error: Not found",
            (true, i) =>
                if machines[i].state == StateT::RUN { machines[i].state = StateT::PAUSE; "Ok" }
                else { "Error: Not running" }               
        };

    String::from(answer)
}

fn hand_stop(args: &str, machines: &mut [Machine; 4]) -> String { 
    if args == "" {
        return String::from("Error: No name");
    }
    
    let answer =
        match find_machine(args, machines) {
            (false, _) => "Error: Not found",
            (true, i) =>
                if machines[i].state == StateT::PAUSE {
                    machines[i].state = StateT::CLOSE;
                    machines[i].name = String::from("");
                    "Ok"
                }
                else { "Error: Not running" }
        };

    String::from(answer)
}

fn hand_enough(args: &str, machines: &mut [Machine; 4]) -> String { 
    if args == "" {
        return String::from("Error: No name");
    }

    let answer =
        match find_machine(args, machines) {
            (false, _) => "Error: Not found",
            (true, i) => {
                machines[i].state = StateT::CLOSE;
                machines[i].name = String::from("");
                "Ok"
            }
        };

    String::from(answer)
}

fn hand_exterminantus(_args: &str, machines: &mut [Machine; 4]) -> String {
    for mut i in machines {
        i.state = StateT::CLOSE;
        i.name = String::from("");
    }

    String::from("Ok")
}

/*fn hand_get(args: &str, machines: &mut [Machine; 4]) -> String { 
}
fn hand_exit_server(args: &str, machines: &mut [Machine; 4]) -> String { String::from(""); }
fn hand_list(args: &str, machines: &mut [Machine; 4]) -> String { String::from(""); }
*/fn hand_strange(_args: &str, _machines: &mut [Machine; 4]) -> String { String::from("Strange query") }
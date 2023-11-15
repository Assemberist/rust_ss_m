#[derive(PartialEq, Copy, Clone)]
pub enum StateT{
    INIT,
    RUN,
    PAUSE,
    CLOSE
}

pub fn state_to_string(state: &StateT) -> &'static str {
	match state {
		StateT::INIT => "Init",
		StateT::RUN => "Run",
		StateT::PAUSE => "Pause",
		StateT::CLOSE => "Close"
	}
}

pub struct Machine{
    pub state:  StateT,
    pub name:   String
}

impl Machine{
    fn new() -> Self {
        Machine {
            state: StateT::CLOSE,
            name: String::new(),
        }
    }
	
	fn update(&mut self, state: StateT) -> Result<&'static str, &'static str> {
		match (self.state.clone(), state) {
			// CLOSE to INIT handled directly in hand_init
			(StateT::INIT, StateT::RUN) => {self.state = state; Ok("Ok")},
			(StateT::RUN, StateT::PAUSE) => {self.state = state; Ok("Ok")},
			(StateT::PAUSE, StateT::RUN) => {self.state = state; Ok("Ok")},
			(StateT::INIT, StateT::CLOSE) => {self.state = StateT::CLOSE;
			                                  self.name = String::new();
                                        	  Ok("Ok")},
			(StateT::PAUSE, StateT::CLOSE) => {self.state = StateT::CLOSE;
			                                   self.name = String::new();
                                        	   Ok("Ok")},
			(old, new) => {
			    if old == new { Err("Already in that state") }
			    else { Err("Wrong state") }
			},
		}
	}
}

pub fn init_statems() -> [Machine; 4] {
    [0; 4].map(|_i| Machine::new())
}

pub fn query(buffer: String, machines: &mut [Machine; 4]) -> String {
    let (head, tail) =
        match buffer.as_str().split_once(':') {
            Some(i) => i,
            None => (buffer.as_str(), ""),
        };
    
	if head == "list" { return hand_list(machines); }

    let handler =
            match head {
                "init" => hand_init,
                "run" => hand_run,
                "pause" => hand_pause,
                "stop" => hand_stop,
                "enough" => hand_enough,
                "exterminantus" => hand_exterminantus,
                "get" => hand_get,
                "exit" => hand_exit_server,
                _other => hand_strange
            };

    match handler(tail, machines) {
		Ok(i) => String::from(i),
		Err(i) => String::from("Error: ") + i,
	}
}

fn find_machine<'a>(name: &str, machines: &'a mut [Machine; 4]) -> Result<&'a mut Machine, &'static str> {
    if name == "" { return Err("No name"); }
	find_machine_2(name, machines)
}

fn find_machine_2<'a>(name: &str, machines: &'a mut [Machine; 4]) -> Result<&'a mut Machine, &'static str> {
    let mut i = 0;
    while i < 4 {
        if machines[i].name == name {
            return Ok(&mut machines[i]);
        }
        i = i+1;
    }
    Err("Machine not found")
}

fn hand_init(args: &str, machines: &mut [Machine; 4]) -> Result<&'static str, &'static str> {
    match find_machine(args, machines) {
        Err("Machine not found") =>
            match find_machine_2("", machines) {
                Err(_) => Err("Error: No free machine"),

                Ok(mach) => {
                    mach.name = String::from(args);
                    mach.state = StateT::INIT;
                    Ok("Ok")
                }
            },

        Ok(_) => Err("Already exist"),
		Err(i) => Err(i),
    }
}

fn set_machine_run(machine: &mut Machine) -> Result<&'static str, &'static str> { machine.update(StateT::RUN) }
fn set_machine_pause(machine: &mut Machine) -> Result<&'static str, &'static str> { machine.update(StateT::PAUSE) }
fn set_machine_close(machine: &mut Machine) -> Result<&'static str, &'static str> { machine.update(StateT::CLOSE) }
fn get_machine(machine: &mut Machine) -> Result<&'static str, &'static str> { Ok(state_to_string(&machine.state)) }

fn hand_run(args: &str, machines: &mut [Machine; 4]) -> Result<&'static str, &'static str> {
	find_machine(args, machines).and_then(set_machine_run)
}

fn hand_pause(args: &str, machines: &mut [Machine; 4]) -> Result<&'static str, &'static str> {
	find_machine(args, machines).and_then(set_machine_pause)
}

fn hand_stop(args: &str, machines: &mut [Machine; 4]) -> Result<&'static str, &'static str> {
	find_machine(args, machines).and_then(set_machine_close)
}

fn hand_enough(args: &str, machines: &mut [Machine; 4]) -> Result<&'static str, &'static str> {
	match find_machine(args, machines) {
		Ok(mach) => {
		    mach.name = String::from("");
			mach.state = StateT::CLOSE;
			Ok("Ok")
		},
		Err(i) => Err(i),
	}
}

fn hand_exterminantus(_args: &str, machines: &mut [Machine; 4]) -> Result<&'static str, &'static str> {
    *machines = init_statems();
	Ok("Ok")
}

fn hand_get(args: &str, machines: &mut [Machine; 4]) -> Result<&'static str, &'static str> {
    find_machine(args, machines).and_then(get_machine)
}

fn hand_list(machines: &[Machine; 4]) -> String {
    let mut acc = String::new();

    for i in machines {
        acc = acc +
                i.name.clone().as_str() +
                " -> " +
                state_to_string(&i.state) +
                "\n";
    }
    
	acc
}

fn hand_exit_server(_args: &str, _machines: &mut [Machine; 4]) -> Result<&'static str, &'static str> { Ok("Bue-bue") }
fn hand_strange(_args: &str, _machines: &mut [Machine; 4]) -> Result<&'static str, &'static str> { Err("Strange query") }

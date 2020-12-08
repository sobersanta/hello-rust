mod deps {
    pub fn run_deps_manager() {
        let mut storage: HashMap<String, Vec<String>> = HashMap::new();

        loop {
            print_prompt();
            let mut input = read_line();

            if let Some(cmd) = input.get(0) {
                if cmd.eq_ignore_ascii_case("add") {
                    if input.len() != 4 || input[2].ne("to") {
                        println!("`Add` command format: `Add Somebody to SomeDepartment`");
                    } else {
                        let employees = storage.entry(input.remove(3)).or_insert(Vec::new());
                        employees.push(input.remove(1));
                        employees.sort();
                    }
                } else if cmd.eq_ignore_ascii_case("list") {
                    if input.len() == 1 {
                        let mut keys: Vec<_> = storage.keys().collect();
                        keys.sort();
                        for department in keys {
                            let employees = &storage[department];
                            print_department(department, employees);
                        }
                    } else {
                        let departments = &input[1..];
                        for department in departments {
                            if let Some(employees) = storage.get(department) {
                                print_department(department, employees);
                            } else {
                                println!("{}: Empty", department);
                            }
                        }
                    }
                } else if cmd.eq_ignore_ascii_case("exit") || cmd.eq_ignore_ascii_case("quit") {
                    break;
                } else {
                    println!("Unknown command");
                }
            }
        }
    }

    fn print_department(department: &String, employees: &Vec<String>) {
        println!("{}:", department);
        for employee in employees {
            println!("    {}", employee);
        }
    }

    fn read_line() -> Vec<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.split_whitespace().map(String::from).collect()
    }

    fn print_prompt() {
        print!("# ");
        io::stdout().flush().expect("stdout flush failed!");
    }
}
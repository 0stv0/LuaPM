pub fn preload_code(raw: &String) -> String {
    let mut lines: Vec<String> = raw.lines().map(|l| l.to_string()).collect();
    preload_operators(&mut lines);

    lines.join("\n")
}

fn preload_operators(lines: &mut Vec<String>) {
    for i in 0..lines.len() {
        let line = lines[i].clone();
        if line.contains("+=") {
            let mut parts = line.split("+=");

            let var    = &parts.next().unwrap().trim();
            let amount = &parts.next().unwrap().trim();

            let fix  = format!("{} = {} + {}", var, var, amount);
            lines[i] = fix;
        } else if line.contains("-=") {
            let mut parts = line.split("-=");

            let var    = &parts.next().unwrap().trim();
            let amount = &parts.next().unwrap().trim();

            let fix  = format!("{} = {} - {}", var, var, amount);
            lines[i] = fix;
        } else if line.contains("*=") {
            let mut parts = line.split("*=");

            let var    = &parts.next().unwrap().trim();
            let amount = &parts.next().unwrap().trim();

            let fix  = format!("{} = {} * {}", var, var, amount);
            lines[i] = fix;
        } else if line.contains("/=") {
            let mut parts = line.split("/=");

            let var    = &parts.next().unwrap().trim();
            let amount = &parts.next().unwrap().trim();

            let fix  = format!("{} = {} / {}", var, var, amount);
            lines[i] = fix;
        } else if line.contains("%=") {
            let mut parts = line.split("%=");

            let var    = &parts.next().unwrap().trim();
            let amount = &parts.next().unwrap().trim();

            let fix  = format!("{} = {} % {}", var, var, amount);
            lines[i] = fix;
        }
    }
}
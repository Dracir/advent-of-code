use crate::valve::*;

pub fn parse_valves(input_example1: String) -> Vec<Valve> {
    let mut valves = Vec::<Valve>::new();

    for line in input_example1.lines() {
        // line example
        // Valve GT has flow rate=0; tunnels lead to valves FJ, AW

        // let mut valve = Valve::new();
        let mut tokens = line.split_whitespace();
        let name = tokens.next().unwrap().to_string();
        tokens.next(); // has
        tokens.next(); // flow
        tokens.next(); // rate=
        let flow_rate = tokens.next().unwrap().parse::<f64>().unwrap();
        tokens.next(); // ;
        tokens.next(); // tunnels
        tokens.next(); // lead
        tokens.next(); // to
        tokens.next(); // valves
        let mut tunnels = Vec::<String>::new();
        for token in tokens {
            tunnels.push(token.to_string());
        }
        let tunnels = tunnels;

        let valve = Valve {
            name,
            flow_rate,
            tunnels,
        };
        valves.push(valve);
    }

    return valves;
}

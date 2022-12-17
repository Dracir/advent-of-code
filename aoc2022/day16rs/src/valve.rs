//Valve GT has flow rate=0; tunnels lead to valves FJ, AW

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Valve {
    pub name: String,
    pub flow_rate: i64,
    pub tunnels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValveBit {
    pub name: String,
    pub id: usize,
    pub flow_rate: i64,
    pub tunnels: Vec<usize>,
}

#[derive(Debug)]
pub struct Problem {
    pub processor_count: usize,
    pub task_count: usize,
    pub edge_count: usize,
    pub comm_rate: Vec<Vec<f64>>,
    pub comp_cost: Vec<Vec<f64>>,
    pub trans_data_vol: Vec<(usize, usize, f64)>,
}
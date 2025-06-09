use plotters::prelude::*;

pub fn plot_convergence(costs: &[f64], output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_cost = costs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let min_cost = costs.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let margin = (max_cost - min_cost) * 0.1;

    let n = costs.len();
    let mut chart = ChartBuilder::on(&root)
        .caption("收斂曲線", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..n, (min_cost - margin)..(max_cost + margin))?;

    chart
        .configure_mesh()
        .x_desc("迭代次數")
        .y_desc("成本")
        .draw()?;

    chart.draw_series(LineSeries::new(
        costs.iter().enumerate().map(|(x, &y)| (x, y)),
        &RED,
    ))?;

    Ok(())
} 
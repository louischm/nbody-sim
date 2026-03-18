use plotters::{chart, prelude::*};
use std::error::Error;

pub fn plot_orbits(csv_path: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(csv_path)?;

    let mut data = std::collections::HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let body = record[1].to_string();
        let x: f64 = record[2].parse()?;
        let y: f64 = record[3].parse()?;

        data.entry(body).or_insert(Vec::new()).push((x, y));
    }

    let root = BitMapBackend::new(output, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption("Orbit SImulation", ("sans-serif", 30))
        .build_cartesian_2d(-2.0e11..2.0e11, -2.0e11..2.0e11)?;

    chart.configure_mesh().draw()?;

    for (i, (body,points)) in data.iter().enumerate() {
        chart
            .draw_series(LineSeries::new(points.clone(), &Palette99::pick(i)))?
            .label(body)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], &Palette99::pick(i))
            });
    }

    chart.configure_series_labels().draw()?;

    Ok(())
}
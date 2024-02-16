use crate::model::stats::graph::graph_builder::GraphBuilder;
use plotters::backend::SVGBackend;
use plotters::drawing::IntoDrawingArea;
use plotters::element::Pie;
use plotters::prelude::{IntoFont, TextStyle};
use plotters::style::WHITE;
use std::error::Error;

pub fn create_pie(builder: &GraphBuilder) -> Result<(), Box<dyn Error>> {
    let path = builder.get_path();
    let root_area = SVGBackend::new(&path, (builder.width, builder.height)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let title = builder.get_title();
    let title_style = TextStyle::from(("Inter", 28).into_font());
    let title = title.as_str();
    root_area.titled(title, title_style)?.margin(20, 20, 20, 20);

    let dims = root_area.dim_in_pixel();
    let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
    let radius = (builder.height as f64) * 0.35;

    let (data, _) = builder.filter();
    let data = data
        .first()
        .unwrap_or(&vec![0])
        .iter()
        .map(|e| *e as f64)
        .collect::<Vec<f64>>();
    let mut labels = builder.get_labels();
    let colors = builder.get_colors();

    let mut data = aggregate(data.to_vec(), &mut labels, 0.05);
    (data, labels) = sort(data, labels);

    let mut pie = Pie::new(&center, &radius, &data, colors, &labels);
    pie.start_angle(270.0);
    pie.percentages(
        ("Inter", radius * 0.1)
            .into_font()
            .color(&colors[colors.len() - 1]),
    );
    pie.label_style(("Inter", radius * 0.1).into_font());
    pie.label_offset(20.0);

    root_area.draw(&pie)?;

    Ok(())
}

fn aggregate(mut data: Vec<f64>, labels: &mut Vec<String>, threshold: f64) -> Vec<f64> {
    data.push(0.0);
    labels.push("Autres".to_string());

    let sum = data.iter().sum::<f64>();
    let mut len = data.len();

    for i in (0..len).rev() {
        if data[i] < 0.01 {
            labels.remove(i);
            data.remove(i);
            len -= 1;
            continue;
        }
        if data[i] / sum < threshold {
            data[0] += data[i];
            labels.remove(i);
            data.remove(i);
            len -= 1;
        }
    }

    data
}

fn sort(data: Vec<f64>, labels: Vec<String>) -> (Vec<f64>, Vec<String>) {
    let mut data_labels = data.into_iter().zip(labels).collect::<Vec<(f64, String)>>();
    data_labels.sort_by(|(cmp_val, _), (val, _)| val.partial_cmp(cmp_val).unwrap());

    let mut data = vec![];
    let mut labels = vec![];
    for (val, label) in data_labels.into_iter() {
        data.push(val);
        labels.push(label.clone());
    }
    (data, labels)
}

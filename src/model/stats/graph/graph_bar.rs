use crate::model::stats::graph::graph_builder::GraphBuilder;
use plotters::backend::SVGBackend;
use plotters::chart::ChartBuilder;
use plotters::coord::Shift;
use plotters::drawing::{DrawingArea, IntoDrawingArea};
use plotters::prelude::{Histogram, IntoFont, IntoSegmentedCoord, WHITE};
use plotters::style::{Color, RGBColor};
use std::error::Error;

pub fn create_histogram(builder: &GraphBuilder, is_vertical: bool) -> Result<(), Box<dyn Error>> {
    let path = builder.get_path();
    let root = SVGBackend::new(&path, (builder.width, builder.height)).into_drawing_area();
    root.fill(&WHITE)?;
    let labels = builder.get_labels();
    let title = builder.get_title();
    let max = (builder.get_max_value() * 1.2) as u32;
    let (data, _) = builder.filter();
    let default = vec![0];
    let data: &Vec<u32> = data.first().unwrap_or(&default);


    if is_vertical {
        draw_vertical_histogram(&root, title, labels, max, data, builder.get_colors())?;
    } else {
        draw_horizontal_histogram(&root, title, labels, max, data, builder.get_colors())?;
    }

    root.present()?;

    Ok(())
}

fn draw_vertical_histogram(
    root: &DrawingArea<SVGBackend, Shift>,
    title: String,
    labels: Vec<String>,
    max: u32,
    data: &[u32],
    colors: &[RGBColor],
) -> Result<(), Box<dyn Error>> {
    let mut chart = ChartBuilder::on(root)
        .x_label_area_size(20)
        .y_label_area_size(20)
        .margin(20)
        .caption(title, ("Inter", 28.0).into_font())
        .build_cartesian_2d(labels.into_segmented(), 0..max)?;

    chart
        .configure_mesh()
        .y_max_light_lines(1)
        .disable_x_mesh()
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .margin(100 / labels.len() as u32)
            .data(
                data.iter()
                    .enumerate()
                    .map(|(index, value)| (&labels[index], *value)),
            )
            .style(colors[0].filled()),
    )?;
    Ok(())
}

fn draw_horizontal_histogram(
    root: &DrawingArea<SVGBackend, Shift>,
    title: String,
    labels: Vec<String>,
    max: u32,
    data: &[u32],
    colors: &[RGBColor],
) -> Result<(), Box<dyn Error>> {
    let mut chart = ChartBuilder::on(root)
        .x_label_area_size(20)
        .y_label_area_size(get_max_char_len(&labels) * 5 + 20)
        .margin(20)
        .caption(title, ("Inter", 30.0).into_font())
        .build_cartesian_2d(0..max, labels.into_segmented())?;

    chart
        .configure_mesh()
        .x_max_light_lines(1)
        .disable_y_mesh()
        .draw()?;

    chart.draw_series(
        Histogram::horizontal(&chart)
            .margin(50 / labels.len() as u32)
            .data(
                data.iter()
                    .enumerate()
                    .map(|(index, value)| (&labels[index], *value)),
            )
            .style(colors[0].filled()),
    )?;
    Ok(())
}

fn get_max_char_len(labels: &[String]) -> u32 {
    labels
        .iter()
        .fold(0, |acc, x| if x.len() > acc as usize { x.len() } else { acc as usize } as u32)
}

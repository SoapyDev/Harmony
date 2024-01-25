use crate::model::stats::graph::graph_builder::GraphBuilder;
use plotters::backend::SVGBackend;
use plotters::chart::{ChartBuilder, LabelAreaPosition, SeriesLabelPosition};
use plotters::element::PathElement;
use plotters::prelude::{AsRelative, IntoDrawingArea, IntoFont, LineSeries, Palette, WHITE};
use plotters::style::{Color, Palette100, BLACK};
use std::error::Error;

pub fn create_line(builder: &GraphBuilder) -> Result<(), Box<dyn Error>> {
    let (from, to) = builder.duration.get_dates();
    let max = (builder.get_max_value() * 1.7) as u32;
    let labels = builder.get_labels();
    let path = builder.get_path();

    let root_area = SVGBackend::new(&path, (builder.width, builder.height)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption(builder.get_title(), ("Inter", 28.0).into_font())
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 10.percent())
        .set_label_area_size(LabelAreaPosition::Right, 10.percent())
        .build_cartesian_2d(from..to, 0..max)?;

    chart
        .configure_mesh()
        .x_labels(12)
        .y_labels(10)
        .x_label_style(("Inter", 14).into_font())
        .y_label_style(("Inter", 14).into_font())
        .y_desc("Beneficiaires")
        .x_max_light_lines(1)
        .y_max_light_lines(1)
        .draw()?;

    let (data, dates) = builder.filter();

    for (i, values) in data.iter().enumerate() {
        let color = Palette100::pick(i);
        let label = labels.get(i % labels.len());
        let label = label.unwrap().as_str();
        chart
            .draw_series(LineSeries::new(
                values
                    .iter()
                    .map(|x| (*dates.get(i).expect("Could not get date"), *x)),
                &color,
            ))?
            .label(label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
    }

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.filled())
        .border_style(BLACK.mix(0.5))
        .legend_area_size(50)
        .draw()?;

    Ok(())
}

use crate::model::stats::graph::date_range::DateRange;
use crate::model::stats::graph::graph_type::GraphType;
use crate::model::stats::graph::time_filter::TimeFilter;
use crate::model::stats::graph::{
    graph_bar::create_histogram, graph_line::create_line, graph_pie::create_pie,
};
use crate::model::stats::stats::StatType;
use chrono::NaiveDate;
use plotters::prelude::RGBColor;
use std::error::Error;
use std::path::{Path, PathBuf};

type GraphData = (Vec<Vec<u32>>, Vec<NaiveDate>);
pub(crate) struct GraphBuilder {
    pub(crate) data: StatType,
    graph: GraphType,
    pub(crate) duration: DateRange,
    filter: TimeFilter,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl GraphBuilder {
    pub(crate) fn new(
        data: StatType,
        graph: GraphType,
        duration: DateRange,
        filter: TimeFilter,
    ) -> Self {
        Self {
            data,
            graph,
            duration,
            filter,
            width: 600,
            height: 450,
        }
    }

    pub(crate) fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        match self.graph {
            GraphType::VerticalBar(_) => create_histogram(self, true),
            GraphType::HorizontalBar(_) => create_histogram(self, false),
            GraphType::Line(_) => create_line(self),
            GraphType::Pie(_) => create_pie(self),
        }
    }
    pub(crate) fn filter(&self) -> GraphData {
        let data = self.duration.filter(self.data.get_data());
        let filtered_data = self.filter.filter(data);
        self.graph.format(filtered_data)
    }
    pub(crate) fn get_max_value(&self) -> f64 {
        self.data.get_max().1 as f64
    }
    pub(crate) fn get_title(&self) -> String {
        format!(
            "Distribution des bénéficiaires par {}",
            self.data.get_type()
        )
    }
    fn get_file_name(&self) -> String {
        format!(
            "{}-{}-{}-{}.svg",
            self.graph,
            self.duration,
            self.filter,
            self.data.get_type()
        )
    }

    #[cfg(target_os = "windows")]
    pub(crate) fn get_path(&self) -> PathBuf {
        let file_name = self.get_file_name();
        Path::new(".\\graph\\").join(file_name)
    }
    #[cfg(target_os = "linux")]
    pub(crate) fn get_path(&self) -> PathBuf {
        let file_name = self.get_file_name();
        let path = Path::new("./graph/").join(file_name);
        path
    }
    pub(crate) fn get_labels(&self) -> Vec<String> {
        self.data.get_labels()
    }
    pub(crate) fn get_colors(&self) -> &[RGBColor] {
        self.graph.get_colors()
    }
}

use crate::model::stats::stats::DateData;
use chrono::NaiveDate;
use plotters::style::RGBColor;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum GraphType {
    VerticalBar(ColorPalette),
    HorizontalBar(ColorPalette),
    Line(ColorPalette),
    Pie(ColorPalette),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ColorPalette {
    Viridris,
    Plasma,
    PastelA,
    PastelB,
    Excel,
    Vivid,
    AuxSources,
    Paired,
}

impl ColorPalette {
    pub(crate) fn get_colors(&self) -> &[RGBColor] {
        match self {
            ColorPalette::Viridris => &[
                RGBColor(43, 11, 88),
                RGBColor(34, 45, 106),
                RGBColor(21, 73, 110),
                RGBColor(12, 95, 111),
                RGBColor(3, 124, 106),
                RGBColor(22, 148, 92),
                RGBColor(89, 174, 53),
                RGBColor(119, 182, 34),
                RGBColor(186, 191, 0),
                RGBColor(255, 255, 255),
            ],
            ColorPalette::Plasma => &[
                RGBColor(0, 0, 106),
                RGBColor(40, 0, 126),
                RGBColor(173, 0, 135),
                RGBColor(127, 0, 123),
                RGBColor(173, 47, 86),
                RGBColor(201, 89, 55),
                RGBColor(215, 128, 29),
                RGBColor(216, 167, 11),
                RGBColor(207, 206, 11),
                RGBColor(255, 255, 255),
            ],
            ColorPalette::PastelA => &[
                RGBColor(255, 147, 141),
                RGBColor(147, 171, 192),
                RGBColor(171, 200, 163),
                RGBColor(188, 169, 193),
                RGBColor(218, 183, 134),
                RGBColor(219, 219, 170),
                RGBColor(194, 182, 156),
                RGBColor(217, 184, 201),
                RGBColor(207, 207, 207),
                RGBColor(0, 0, 0),
            ],
            ColorPalette::PastelB => &[
                RGBColor(146, 191, 171),
                RGBColor(217, 171, 140),
                RGBColor(169, 179, 197),
                RGBColor(209, 168, 193),
                RGBColor(195, 210, 167),
                RGBColor(219, 207, 141),
                RGBColor(206, 191, 170),
                RGBColor(170, 170, 170),
                RGBColor(0, 0, 0),
            ],
            ColorPalette::Excel => &[
                RGBColor(68, 114, 196),
                RGBColor(237, 125, 49),
                RGBColor(165, 165, 165),
                RGBColor(255, 192, 0),
                RGBColor(91, 155, 213),
                RGBColor(112, 173, 71),
                RGBColor(38, 68, 120),
                RGBColor(158, 72, 14),
                RGBColor(153, 115, 0),
                RGBColor(0, 0, 0),
            ],
            ColorPalette::Paired => &[
                RGBColor(134, 172, 192),
                RGBColor(4, 90, 147),
                RGBColor(145, 189, 107),
                RGBColor(23, 128, 17),
                RGBColor(215, 122, 121),
                RGBColor(192, 0, 1),
                RGBColor(217, 158, 81),
                RGBColor(219, 96, 0),
                RGBColor(168, 145, 180),
                RGBColor(76, 33, 122),
                RGBColor(0, 0, 0),
            ],
            ColorPalette::AuxSources => &[
                RGBColor(4, 180, 228),
                RGBColor(123, 156, 4),
                RGBColor(4, 44, 84),
                RGBColor(64, 184, 196),
                RGBColor(64, 116, 100),
                RGBColor(68, 44, 100),
                RGBColor(4, 44, 124),
                RGBColor(255, 255, 255),
            ],
            ColorPalette::Vivid => &[
                RGBColor(96, 167, 96),
                RGBColor(157, 141, 178),
                RGBColor(217, 159, 103),
                RGBColor(219, 219, 121),
                RGBColor(28, 78, 143),
                RGBColor(205, 0, 96),
                RGBColor(158, 62, 0),
                RGBColor(72, 72, 72),
                RGBColor(0, 0, 0),
            ],
        }
    }
    pub(crate) fn get_first_color(&self) -> &[RGBColor] {
        self.get_colors().get(0..1).unwrap_or(&[])
    }

    pub(crate) fn get_labels() -> Vec<String> {
        vec![
            "Viridris".to_string(),
            "Plasma".to_string(),
            "Pastel-1".to_string(),
            "Pastel-2".to_string(),
            "Excel".to_string(),
            "Vivid".to_string(),
            "Aux-sources".to_string(),
            "Paire".to_string(),
        ]
    }
}

impl Display for ColorPalette {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorPalette::Viridris => write!(f, "Viridris"),
            ColorPalette::Plasma => write!(f, "Plasma"),
            ColorPalette::PastelA => write!(f, "Pastel-1"),
            ColorPalette::PastelB => write!(f, "Pastel-2"),
            ColorPalette::Excel => write!(f, "Excel"),
            ColorPalette::Vivid => write!(f, "Vivid"),
            ColorPalette::AuxSources => write!(f, "Aux-sources"),
            ColorPalette::Paired => write!(f, "Paire"),
        }
    }
}

impl FromStr for ColorPalette {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Viridris" => Ok(ColorPalette::Viridris),
            "Plasma" => Ok(ColorPalette::Plasma),
            "Pastel-1" => Ok(ColorPalette::PastelA),
            "Pastel-2" => Ok(ColorPalette::PastelB),
            "Excel" => Ok(ColorPalette::Excel),
            "Vivid" => Ok(ColorPalette::Vivid),
            "Aux-sources" => Ok(ColorPalette::AuxSources),
            "Paire" => Ok(ColorPalette::Paired),
            _ => Err(anyhow::anyhow!("Unknown color palette")),
        }
    }
}
impl Display for GraphType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphType::VerticalBar(e) => write!(f, "{}-bar-v", e),
            GraphType::HorizontalBar(e) => write!(f, "{}-bar-h", e),
            GraphType::Line(e) => write!(f, "{}-ligne", e),
            GraphType::Pie(e) => write!(f, "{}-tarte", e),
        }
    }
}
type GraphData = (Vec<Vec<u32>>, Vec<NaiveDate>);
impl GraphType {
    pub(crate) fn format(&self, data: Vec<DateData>) -> GraphData {
        match self {
            GraphType::VerticalBar(_) => self.format_avg(data),
            GraphType::HorizontalBar(_) => self.format_avg(data),
            GraphType::Line(_) => self.format_line(data),
            GraphType::Pie(_) => self.format_avg(data),
        }
    }

    pub(crate) fn get_colors(&self) -> &[RGBColor] {
        match self {
            GraphType::VerticalBar(palette) => palette.get_first_color(),
            GraphType::HorizontalBar(palette) => palette.get_first_color(),
            GraphType::Line(palette) => palette.get_colors(),
            GraphType::Pie(palette) => palette.get_colors(),
        }
    }
    fn format_avg(&self, data: Vec<DateData>) -> GraphData {
        let default = DateData::default();
        let value = data.first().unwrap_or(&default);
        let len_width = value.values.len();
        let len_height = data.len();
        let avg: Vec<u32> = data.iter().fold(vec![0; len_width], |acc, e| {
            acc.iter()
                .zip(e.values.iter())
                .map(|(a, b)| a + b)
                .collect::<Vec<u32>>()
        });
        let avg = avg
            .iter()
            .map(|e| e / len_height as u32)
            .collect::<Vec<u32>>();

        (vec![avg], vec![value.date])
    }

    fn format_line(&self, data: Vec<DateData>) -> GraphData {
        data.into_iter().fold((vec![], vec![]), |mut acc, e| {
            acc.0.push(e.values);
            acc.1.push(e.date);
            acc
        })
    }

    pub(crate) fn from_str(value: &str, palette: &ColorPalette) -> Self {
        match value {
            "Histogramme-Vertical" => GraphType::VerticalBar(*palette),
            "Histogramme-Horizontal" => GraphType::HorizontalBar(*palette),
            "Ligne" => GraphType::Line(*palette),
            "Tarte" => GraphType::Pie(*palette),
            _ => GraphType::VerticalBar(*palette),
        }
    }

    pub(crate) fn as_string(&self) -> String {
        match self {
            GraphType::VerticalBar(_) => "Histogramme-Vertical".to_string(),
            GraphType::HorizontalBar(_) => "Histogramme-Horizontal".to_string(),
            GraphType::Line(_) => "Ligne".to_string(),
            GraphType::Pie(_) => "Tarte".to_string(),
        }
    }

    pub(crate) fn get_labels() -> Vec<String> {
        vec![
            "Histogramme-Vertical".to_string(),
            "Histogramme-Horizontal".to_string(),
            "Ligne".to_string(),
            "Tarte".to_string(),
        ]
    }

    pub(crate) fn get_palette(&self) -> ColorPalette {
        match self {
            GraphType::VerticalBar(e) => *e,
            GraphType::HorizontalBar(e) => *e,
            GraphType::Line(e) => *e,
            GraphType::Pie(e) => *e,
        }
    }
}

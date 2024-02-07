use crate::model::stats::graph::date_range::DateRange;
use crate::model::stats::graph::graph_builder::GraphBuilder;
use crate::model::stats::graph::graph_type::{ColorPalette, GraphType};
use crate::model::stats::graph::time_filter::TimeFilter;
use crate::model::stats::statistiques::statistiques::Statistiques;
use crate::model::stats::stats::{StatType, Stats};
use crate::view::inputs::dates::DateMut;
use crate::view::inputs::selectables::SelectInputMut;
use dioxus::prelude::*;
use std::str::FromStr;

#[component]
pub(crate) fn StatsPage(cx: Scope) -> Element {
    let use_stats = use_shared_state::<Stats>(cx).unwrap();

    if !use_stats.read().is_empty() {
        let initial_dates = use_stats.read().presences.get_max_date_range();
        let dates = use_ref(cx, || DateRange::new(&initial_dates.0, &initial_dates.1));
        let statistiques = use_state(cx, || Statistiques::from(use_stats.clone()));

        let graph_1 = use_ref(cx, || GraphValues {
            data: StatType::Age(use_stats.read().ages.clone()),
            graph: GraphType::VerticalBar(ColorPalette::AuxSources),
            filter: TimeFilter::Day,
        });

        let graph_2 = use_ref(cx, || GraphValues {
            data: StatType::Income(use_stats.read().incomes.clone()),
            graph: GraphType::Pie(ColorPalette::PastelA),
            filter: TimeFilter::Day,
        });

        let graph_3 = use_ref(cx, || GraphValues {
            data: StatType::Kid(use_stats.read().kids.clone()),
            graph: GraphType::Pie(ColorPalette::AuxSources),
            filter: TimeFilter::Day,
        });

        let graph_4 = use_ref(cx, || GraphValues {
            data: StatType::Sexe(use_stats.read().sexes.clone()),
            graph: GraphType::Pie(ColorPalette::Viridris),
            filter: TimeFilter::Day,
        });

        render! {
            style{
                include_str!("../../../assets/style/stats.css"),
            }
            div {
                main{
                    class: "stats-grid",
                    div{
                        class: "stats-container",
                        div{
                            class: "date-row",
                            DateMut{
                                on_change: |e : FormEvent| dates.with_mut(|dates| dates.set_from(&e.value)),
                                value: dates.read().get_from(),
                                label: "Du",
                                class: "date-input",
                            },
                            DateMut{
                                on_change: |e: FormEvent| dates.with_mut(|dates| dates.set_to(&e.value)),
                                value: dates.read().get_to(),
                                label: "Au",
                                class: "date-input",
                            },

                        }
                        div{
                            class: "stats-title",
                            h3{"Bénéficiaires"}
                            div{
                                class: "stats-beneficiary",
                                ul{
                                    class: "stats-values",
                                    li{"Totaux : {statistiques.total}"}
                                    li{"Totaux actifs : {statistiques.active}"}
                                    li{"Nouveaux : {statistiques.visits}"}
                                }
                            }
                        }
                        div{
                            class: "stats-title",
                            h3{"Personna"}
                            div{
                                class: "stats-persona",
                                ul{
                                    li{"Âge : {statistiques.age}"}
                                    li{"Sexe : {statistiques.sexe}"}
                                    li{"Langue : {statistiques.language}"}
                                    li{"Origine : {statistiques.origin}"}
                                    li{"Situation familiale : {statistiques.family_situation}"}
                                    li{"Enfants : {statistiques.kid}"}
                                    li{"Ville : {statistiques.city}"}
                                    li{"Éducation : {statistiques.study}"}
                                    li{"Emploi : {statistiques.employment}"}
                                    li{"Tranche salariale : {statistiques.income}"}
                                }
                            }
                        }
                    }
                    render!{
                        div{
                            class: "stats-graphs",
                            div{
                                class: "img-row",
                                div{
                                    class: "img-widget",
                                    Graph{
                                        stats: use_stats,
                                        graph_values: graph_1,
                                        dates: dates,
                                    },
                                },
                                div{
                                    class: "img-widget",
                                   Graph{
                                        stats: use_stats,
                                        graph_values: graph_2,
                                        dates: dates,
                                    },
                                },
                            },
                            div{
                                class: "img-row",
                                div{
                                    class: "img-widget",
                                   Graph{
                                        stats: use_stats,
                                        graph_values: graph_3,
                                        dates: dates,
                                    },
                                }
                                div{
                                    class: "img-widget",
                                    Graph{
                                        stats: use_stats,
                                        graph_values: graph_4,
                                        dates: dates,
                                    },
                                },
                            },
                        }
                    }
                }
            }
        }
    } else {
        render! {
            div{
                main{
                    div{
                        h1{"Chargement des statistiques"}
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct GraphValues {
    data: StatType,
    graph: GraphType,
    filter: TimeFilter,
}
#[component]
fn Graph<'a>(
    cx: Scope,
    stats: &'a UseSharedState<Stats>,
    graph_values: &'a UseRef<GraphValues>,
    dates: &'a UseRef<DateRange>,
) -> Element {
    let mut builder = GraphBuilder::new(
        graph_values.read().data.clone(),
        graph_values.read().graph,
        *dates.read(),
        graph_values.read().filter,
    );
    builder.draw().expect("Error while drawing graph");
    let path = builder.get_path();
    let path = path.to_str().unwrap();
    render! {
        div{
            class: "img-inputs",
            SelectInputMut{
                on_change: |e : FormEvent| graph_values.with_mut(|val| val.graph = GraphType::from_str(&e.value, &val.graph.get_palette())),
                value: graph_values.read().graph.as_string(),
                values: GraphType::get_labels(),
                list: GraphType::get_labels(),
                is_multiple: false,
                label: "Graphique",
                class: "graph-input select",
            }
            SelectInputMut{
                on_change: |e: FormEvent| graph_values.with_mut(|val| val.graph = GraphType::from_str(&val.graph.as_string(), &ColorPalette::from_str(&e.value).unwrap_or(ColorPalette::AuxSources))),
                value: graph_values.read().graph.get_palette().to_string(),
                values: ColorPalette::get_labels(),
                list: ColorPalette::get_labels(),
                is_multiple: false,
                label: "Palette",
                class: "graph-input select",
            }
            SelectInputMut{
                on_change: |e: FormEvent| graph_values.with_mut(|val| val.data = StatType::new(stats, &e.value)),
                value: graph_values.read().data.get_value(),
                values: Stats::get_labels(),
                list: Stats::get_labels(),
                is_multiple: false,
                label: "Données",
                class: "graph-input select",
            }
            SelectInputMut{
                on_change: |e: FormEvent| graph_values.with_mut(|val| val.filter = TimeFilter::from_str(&e.value).unwrap_or(TimeFilter::Day)),
                value: graph_values.read().filter.to_string(),
                values: TimeFilter::get_labels(),
                list: TimeFilter::get_labels(),
                is_multiple: false,
                label: "Unité de temps",
                class: "graph-input select",
            }
        }

        img{
            src: "{path}",
            width: "{builder.width}",
            height: "{builder.height}",
        }
    }
}

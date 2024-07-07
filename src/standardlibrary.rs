use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::drawing::IntoDrawingArea;
use plotters::element::PathElement;
use plotters::series::LineSeries;
use plotters::style::{full_palette::*, Color, IntoFont, MAGENTA};
use rand::{seq::SliceRandom, thread_rng};
use std::io::{stdin, stdout, Write};
use viuer::{print_from_file, Config};

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    data::{sizedset::SizedSet, Data},
    interpreter::{Interpreter, Std},
};
#[derive(Debug, Clone)]
pub struct StandardLibrary {
    pub map: Std,
}

impl StandardLibrary {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn from_map(map: Std) -> Self {
        Self { map }
    }

    pub fn init_std(&mut self) {
        self.map.insert("print".to_string(), |x, _, _, _| {
            x.iter().for_each(|a| println!("{a}"));
            Data::default()
        });
        self.map.insert("read".to_string(), |_, _, _, _| {
            print!("Enter number: ");
            stdout().flush().unwrap();
            let mut buf = String::new();

            stdin().read_line(&mut buf).unwrap();

            Data::Number(buf.trim_end().parse::<f32>().unwrap())
        });

        self.map.insert("round".to_string(), |x, _, _, _| {
            Data::Number(x[0].to_number().round())
        });

        self.map.insert("ceil".to_string(), |x, _, _, _| {
            Data::Number(x[0].to_number().ceil())
        });

        self.map.insert("floor".to_string(), |x, _, _, _| {
            Data::Number(x[0].to_number().floor())
        });

        self.map.insert("log".to_string(), |x, _, _, _| {
            Data::Number(x[0].to_number().ln())
        });

        self.map.insert("sin".to_string(), |x, _, _, _| {
            Data::Number(x[0].to_number().sin())
        });
        self.map.insert("cos".to_string(), |x, _, _, _| {
            Data::Number(x[0].to_number().cos())
        });
        self.map.insert("tan".to_string(), |x, _, _, _| {
            Data::Number(x[0].to_number().tan())
        });

        self.map.insert("sqrt".to_string(), |x, _, _, _| {
            Data::Number(x[0].to_number().sqrt())
        });
        self.map.insert("cbrt".to_string(), |x, _, _, _| {
            Data::Number(x[0].to_number().cbrt())
        });
        self.map.insert("nrt".to_string(), |x, _, _, _| {
            Data::Number(x[0].to_number().powf(1.0 / x[1].to_number()))
        });

        self.map
            .insert("len".to_string(), |x, variables, functions, std| {
                Data::Number(match &x[0] {
                    Data::Number(_) | Data::Function(_) | Data::Bool(_) => 1.0,
                    Data::SizedSet(x) => x.values.len() as f32,
                    Data::UnsizedSet(x) => x.len(&variables, &functions, &std.map),
                })
            });

        self.map
            .insert("get".to_string(), |x, variables, functions, std| {
                x[0].to_set(&variables, &functions, &std.map)
                    .values
                    .get(x[1].to_number() as usize)
                    .unwrap()
                    .clone()
            });
        self.map
            .insert("set".to_string(), |x, variables, functions, std| {
                let mut s = x[0].to_set(&variables, &functions, &std.map).values.clone();
                s.insert(x[1].to_number() as usize, x[2].clone());
                Data::SizedSet(SizedSet::new(s))
            });
        self.map
            .insert("sum".to_string(), |x, variables, functions, std| {
                Data::Number(
                    x.iter()
                        .map(|s| {
                            s.to_set(&variables, &functions, &std.map)
                                .values
                                .iter()
                                .map(|y| y.to_number())
                                .sum::<f32>()
                        })
                        .sum(),
                )
            });

        self.map
            .insert("product".to_string(), |x, variables, functions, std| {
                Data::Number(
                    x.iter()
                        .map(|s| {
                            s.to_set(&variables, &functions, &std.map)
                                .values
                                .iter()
                                .map(|y| y.to_number())
                                .product::<f32>()
                        })
                        .product(),
                )
            });

        self.map
            .insert("map".to_string(), |x, variables, functions, std| {
                let mut sets = vec![];
                let mut i = String::new();
                for arg in x {
                    if matches!(arg, Data::Function(_)) {
                        i = arg.to_function();
                        break;
                    }
                    sets.push(arg);
                }
                if std.map.contains_key(&i) {
                    let f = std.map.get(&i).unwrap();
                    let mut r = vec![];
                    for set in sets {
                        for n in set.to_set(&variables, &functions, &std.map).values.clone() {
                            r.push(f(
                                vec![n],
                                variables.clone(),
                                functions.clone(),
                                StandardLibrary::from_map(std.map.clone()),
                            ));
                        }
                    }
                    return Data::SizedSet(SizedSet::new(r));
                } else if let Data::Function(f) = functions.get(&i).unwrap().clone() {
                    let mut variables = variables.clone();

                    for (i, arg) in f.args.iter().enumerate() {
                        variables.insert(arg.to_string(), sets[i].clone());
                    }

                    return Interpreter::eval_expression(&f.expr, &variables, &functions, &std.map);
                }
                unreachable!()
            });

        self.map
            .insert("graph".to_string(), |x, mut variables, functions, std| {
                let colors = [
                    BLACK, BLUE, BLUE_300, BLUE_600, BLUE_900, RED, RED_300, RED_600, RED_900,
                    GREEN, GREEN_300, GREEN_600, GREEN_900, YELLOW, YELLOW_300, YELLOW_600,
                    YELLOW_900, MAGENTA,
                ];

                let start = SystemTime::now();
                let duration = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
                let name = format!("graph-output-{duration}.png");

                let root = BitMapBackend::new(&name, (640, 480)).into_drawing_area();

                root.fill(&WHITE).unwrap();

                let mut chart = ChartBuilder::on(&root)
                    .caption("Graph output", ("sans-serif", 20).into_font())
                    .margin(5)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(-1f32..1f32, -1f32..1f32)
                    .unwrap();

                chart.configure_mesh().draw().unwrap();

                for z in x {
                    if let Data::Function(f) = functions.get(&z.to_function()).unwrap() {
                        let style = colors.choose(&mut thread_rng()).unwrap_or(&RED);

                        chart
                            .draw_series(LineSeries::new(
                                (-50..=50).map(|x| x as f32 / 50.0).map(|x| {
                                    variables.insert(
                                        f.args.first().unwrap().to_string(),
                                        Data::Number(x),
                                    );
                                    (
                                        x,
                                        Interpreter::eval_expression(
                                            &f.expr, &variables, &functions, &std.map,
                                        )
                                        .to_number(),
                                    )
                                }),
                                &style,
                            ))
                            .unwrap()
                            .label(format!("{f}"))
                            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], *style));
                    }
                }

                chart
                    .configure_series_labels()
                    .background_style(WHITE.mix(0.8))
                    .border_style(BLACK)
                    .draw()
                    .unwrap();

                root.present().unwrap();

                let conf = Config::default();

                let _ = print_from_file(&name, &conf);

                Data::default()
            });
    }
}

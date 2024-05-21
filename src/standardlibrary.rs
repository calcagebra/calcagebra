use std::io::{stdin, stdout, Write};
use textplots::{AxisBuilder, Chart, LineStyle, Plot, Shape, TickDisplay, TickDisplayBuilder};

use std::collections::HashMap;

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

        self.map.insert("get".to_string(), |x, _, _, _| {
            x[0].to_set()
                .values
                .get(x[1].to_number() as usize)
                .unwrap()
                .clone()
        });
        self.map.insert("set".to_string(), |x, _, _, _| {
            let mut s = x[0].to_set().values.clone();
            s.insert(x[1].to_number() as usize, x[2].clone());
            Data::SizedSet(SizedSet::new(s))
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
                if std.map.get(&i).is_some() {
                    let f = std.map.get(&i).unwrap();
                    let mut r = vec![];
                    for set in sets {
                        for n in set.to_set().values.clone() {
                            r.push(f(
                                vec![n],
                                variables.clone(),
                                functions.clone(),
                                StandardLibrary::from_map(std.map.clone()),
                            ));
                        }
                    }
                    return Data::SizedSet(SizedSet::new(r));
                }
                let (args, code) = functions.get(&i).unwrap().clone();

                let mut variables = variables.clone();

                for (i, arg) in args.iter().enumerate() {
                    variables.insert(arg.to_string(), sets[i].clone());
                }

                Interpreter::eval_expression(&code, &variables, &functions, &std.map)
            });

        self.map
            .insert("graph".to_string(), |x, variables, functions, std| {
                let mut binding = Chart::default();
                let mut chart = binding
                    .x_axis_style(LineStyle::Solid)
                    .y_axis_style(LineStyle::Solid)
                    .y_tick_display(TickDisplay::Sparse);
                let mut shapes = vec![];

                for z in x {
                    let (args, expr) = functions.get(&z.to_function()).unwrap();
                    let shape = Shape::Continuous(Box::new(|y| {
                        let mut variables = variables.clone();
                        variables.insert(args.first().unwrap().to_string(), Data::Number(y));
                        Interpreter::eval_expression(expr, &variables, &functions, &std.map)
                            .to_number()
                    }));
                    shapes.push(shape);
                }
                for shape in &shapes {
                    chart = chart.lineplot(shape);
                }
                chart.nice();
                Data::default()
            });
    }
}

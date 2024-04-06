use std::io::{stdin, stdout, Write};
use textplots::{AxisBuilder, Chart, LineStyle, Plot, Shape, TickDisplay, TickDisplayBuilder};

use std::collections::HashMap;

use crate::{
    ast::Expression,
    data::{set::Set, Data},
    interpreter::Interpreter,
};

pub type Variables = HashMap<String, Data>;
pub type Functions = HashMap<String, (Vec<String>, Expression)>;
pub type Std = HashMap<String, Function>;
pub type Function = fn(Vec<Data>, Variables, Functions, StandardLibrary) -> Data;

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

        self.map.insert("len".to_string(), |x, _, _, _| {
            Data::Number(match &x[0] {
                Data::Number(_) | Data::Function(_) => 1.0,
                Data::Set(x) => x.values.len() as f32,
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
            Data::Set(Set::new(s))
        });

        self.map
            .insert("graph".to_string(), |x, variables, functions, std| {
                x.iter().for_each(|f| {
                    let (args, expr) = functions.get(&f.to_function()).unwrap();
                    println!(
                        "\n\x1b[1m{} {} = {}\x1b[0m",
                        f.to_function(),
                        args.join(" "),
                        expr
                    );
                    Chart::new_with_y_range(200, 60, -5.0, 5.0, -5.0, 5.0)
                        .x_axis_style(LineStyle::Solid)
                        .y_axis_style(LineStyle::Solid)
                        .lineplot(&Shape::Continuous(Box::new(|x| {
                            let mut variables = variables.clone();
                            variables.insert(args.first().unwrap().to_string(), Data::Number(x));
                            Interpreter::eval_expression(expr, &variables, &functions, &std.map)
                                .to_number()
                        })))
                        .y_tick_display(TickDisplay::Sparse)
                        .nice();
                });
                Data::default()
            });
    }
}

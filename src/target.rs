use crate::ast::{Expression, Program, Statement};
use std::collections::HashMap;

static mut LABEL_COUNT: i32 = 0;
fn label_count() -> i32 {
    unsafe {
        LABEL_COUNT
    }
}

fn inc_label_count() {
    unsafe {
        LABEL_COUNT += 1;
    }
}


pub struct TI84;
impl Target<String> for TI84 {
    fn process(self, prog: Program) -> String {
        let mut result = String::new();
        for stmt in prog.statements {
            match stmt {
                Statement::ForLoop {
                    variable,
                    begin,
                    end,
                    body,
                } => {
                    result += &format!("For({},{},{},1)\n", variable, begin, end);
                    result += &Self.process(Program::new(body));
                    result += "End\n";
                }
                Statement::WhileLoop { condition, body } => {
                    result += &format!("While ({})\n", condition);
                    result += &Self.process(Program::new(body));
                    result += "End\n";
                }
                Statement::IfThenElse {
                    condition,
                    if_then,
                    if_else,
                } => {
                    result += &format!("If ({})\nThen\n", condition);
                    result += &Self.process(Program::new(if_then));
                    result += "Else\n";
                    result += &Self.process(Program::new(if_else));
                    result += "End\n";
                }
                Statement::Menu(name, cases) => {
                    let mut case_labels = HashMap::new();
                    result += &format!("Menu(\"{}\"", name.to_uppercase());
                    let mut proced = HashMap::new();
                    for (case, body) in &cases {
                        proced.insert(case, Self.process(Program::new(body.clone())));
                        case_labels.insert(case.clone(), label_count());
                        result += &format!(",\"{}\",{}", case.to_uppercase(), label_count());
                        inc_label_count();
                    }
                    result += ")\n";

                    for case in cases.keys() {
                        result += &format!("Lbl {}\n", case_labels[case]);
                        result += &format!("{}", proced[case]);
                        result += &format!("Goto {}\n", label_count());
                    }
                    


                    result += &format!("Lbl {}\n", label_count());
                }
                Statement::Print(vals) => {
                    result += "Disp ";
                    for val in vals {
                        if let Expression::String(s) = val {
                            result += &format!("\"{}\",", s.to_uppercase());
                        } else {
                            result += &format!("{},", val);
                        }
                    }
                    result.pop();
                    result.push('\n');
                }
                Statement::Input(ch) => result += &format!("Input {}\n", ch),
                Statement::Assign(ch, expr) => result += &format!("({})->{}\n", expr, ch),
                Statement::Clear => result += "ClrHome\n",
                Statement::Stop => result += "Stop\n",
                Statement::Pause => result += "Pause\n"
            }
        }
        result
    }
}

pub trait Target<T> {
    fn process(self, prog: Program) -> T;
}

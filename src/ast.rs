use std::{
    collections::HashMap,
    rc::Rc,
    fmt::{Display, Formatter, Error}
};


#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub(crate) statements: Vec<Statement>
}

impl Program {
    pub(crate) fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    ForLoop {
        variable: char,
        begin: Rc<Expression>,
        end: Rc<Expression>,
        body: Vec<Self>
    },
    WhileLoop {
        condition: Expression,
        body: Vec<Self>
    },
    IfThenElse {
        condition: Expression,
        if_then: Vec<Self>,
        if_else: Vec<Self>
    },
    Menu(String, HashMap<String, Vec<Self>>),
    Print(Vec<Expression>),
    Input(char),
    Assign(char, Expression),
    Clear,
    Pause,
    Stop
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Variable(char),
    Number(f64),
    String(String),
    Math(Rc<Math>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::Variable(ch) => write!(f, "{}", ch),
            Self::Number(n) => write!(f, "{}", n),
            Self::String(st) => write!(f, "{}", st),
            Self::Math(m) => write!(f, "{}", m),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Math {
    Add(Expression, Expression),
    Multiply(Expression, Expression),
    Subtract(Expression, Expression),
    Divide(Expression, Expression),
    Negate(Expression),
    Exponent {
        base: Expression,
        power: Expression,
    },
    NaturalLog(Expression),
    Log {
        base: Expression,
        operand: Expression,
    },
    Equal(Expression, Expression),
    NotEqual(Expression, Expression),
    And(Expression, Expression),
    Or(Expression, Expression),
    Not(Expression),
    Sin(Expression),
    Cos(Expression),
    Tan(Expression),
}


impl Display for Math {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::Add(a, b) => write!(f, "({}+{})", a, b),
            Self::Subtract(a, b) => write!(f, "({}-{})", a, b),
            Self::Multiply(a, b) => write!(f, "({}*{})", a, b),
            Self::Divide(a, b) => write!(f, "({}/{})", a, b),
            Self::Negate(a) => write!(f, "(-{})", a),
            Self::Exponent { base, power } => write!(f, "(({})^({}))", base, power),
            Self::NaturalLog(op) => write!(f, "ln({})", op),
            Self::Log { base, operand } => write!(f, "(log({}) / log({}))", operand, base),
            Self::Equal(a, b) => write!(f, "({})=({})", a, b),
            Self::NotEqual(a, b) => write!(f, "((({})=({}))=0)", a, b),
            Self::And(a, b) => write!(f, "({} and {})", a, b),
            Self::Or(a, b) => write!(f, "({} or {})", a, b),
            Self::Not(a) => write!(f, "(({})=0)", a),
            Self::Sin(a) => write!(f, "sin({})", a),
            Self::Cos(a) => write!(f, "cos({})", a),
            Self::Tan(a) => write!(f, "tan({})", a)
        }
    }
}

use crate::algebra::MultiVectorClass;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DataType<'a> {
    Integer,
    SimdVector(usize),
    MultiVector(&'a MultiVectorClass),
}

impl<'a> DataType<'a> {
    pub fn data_class_name(&self) -> String {
        match &self {
            DataType::Integer => "Integer".to_string(),
            DataType::SimdVector(x) => format!("SimdVector{x}"),
            DataType::MultiVector(c) => c.class_name.clone(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UsualGatherData {
    pub negate: bool,
    pub group: usize,
    pub element: usize,
    pub group_size: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum GatherData {
    Usual(UsualGatherData),
    RawZero,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ExpressionContent<'a> {
    None,
    Variable(&'static str),
    InvokeClassMethod(&'a MultiVectorClass, &'static str, Vec<(DataType<'a>, Expression<'a>)>),
    InvokeInstanceMethod(DataType<'a>, Box<Expression<'a>>, &'static str, Vec<(DataType<'a>, Expression<'a>)>),
    Conversion(&'a MultiVectorClass, &'a MultiVectorClass, Box<Expression<'a>>),
    Select(Box<Expression<'a>>, Box<Expression<'a>>, Box<Expression<'a>>),
    Access(Box<Expression<'a>>, usize),
    Swizzle(Box<Expression<'a>>, Vec<usize>),
    Gather(Box<Expression<'a>>, Vec<GatherData>),
    Constant(DataType<'a>, Vec<isize>),
    ConstructVec(DataType<'a>, Vec<Expression<'a>>),
    SquareRoot(Box<Expression<'a>>),
    Add(Box<Expression<'a>>, Box<Expression<'a>>),
    Subtract(Box<Expression<'a>>, Box<Expression<'a>>),
    Multiply(Box<Expression<'a>>, Box<Expression<'a>>),
    Divide(Box<Expression<'a>>, Box<Expression<'a>>),
    LessThan(Box<Expression<'a>>, Box<Expression<'a>>),
    Equal(Box<Expression<'a>>, Box<Expression<'a>>),
    LogicAnd(Box<Expression<'a>>, Box<Expression<'a>>),
    BitShiftRight(Box<Expression<'a>>, Box<Expression<'a>>),
    Pow(Box<Expression<'a>>, Box<Expression<'a>>),
    Exp(Box<Expression<'a>>),
    Sin(Box<Expression<'a>>),
    Cos(Box<Expression<'a>>),
    Tan(Box<Expression<'a>>),
    Sinh(Box<Expression<'a>>),
    Cosh(Box<Expression<'a>>),
    Tanh(Box<Expression<'a>>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Expression<'a> {
    pub size: usize,
    pub data_type_hint: Option<DataType<'a>>,
    pub content: ExpressionContent<'a>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Parameter<'a> {
    pub name: &'static str,
    pub data_type: DataType<'a>,
}

impl<'a> Parameter<'a> {
    pub fn multi_vector_class(&self) -> &'a MultiVectorClass {
        if let DataType::MultiVector(class) = self.data_type {
            class
        } else {
            let n = self.name;
            let d = &self.data_type;
            unreachable!("No multi_vector_class for {n:?} with datatype {d:?}")
        }
    }
}

// TODO overhaul Ast leveraging the insights gained so far, so that we might implement something
//  like complete cargo feature support and pave the way for things like G(4,2,0) and G(6,3,0).
//  This (newer Ast, not "this" Ast right here) is where we should also embrace multithreaded
//  asynchronous code generation, more declarative (async) dependency evaluation (for trait impls
//  depending on trait impls), and code inlining (to remove all inter-trait dependencies in the
//  final output). Should also help make it easier to rearrange the output files.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AstNode<'a> {
    None,
    Preamble,
    TraitDefinition {
        name: String,
        params: usize,
        docs: String,
    },
    TypeAlias(String, String),
    ClassDefinition {
        class: &'a MultiVectorClass,
    },
    ReturnStatement {
        expression: Box<Expression<'a>>,
    },
    VariableAssignment {
        name: &'static str,
        data_type: Option<DataType<'a>>,
        expression: Box<Expression<'a>>,
    },
    IfThenBlock {
        condition: Box<Expression<'a>>,
        body: Vec<AstNode<'a>>,
    },
    WhileLoopBlock {
        condition: Box<Expression<'a>>,
        body: Vec<AstNode<'a>>,
    },
    TraitImplementation {
        result: Parameter<'a>,
        class: &'a MultiVectorClass,
        parameters: Vec<Parameter<'a>>,
        body: Vec<AstNode<'a>>,
    },
}

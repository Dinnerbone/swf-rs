#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Add,
    Add2,
    And,
    AsciiToChar,
    BitAnd,
    BitLShift,
    BitOr,
    BitRShift,
    BitURShift,
    BitXor,
    Call,
    CallFunction,
    CallMethod,
    CastOp,
    CharToAscii,
    CloneSprite,
    ConstantPool(Vec<String>),
    Decrement,
    DefineFunction {
        name: String,
        params: Vec<String>,
        actions: Vec<Action>,
    },
    DefineFunction2(Function),
    DefineLocal,
    DefineLocal2,
    Delete,
    Delete2,
    Divide,
    EndDrag,
    Enumerate,
    Enumerate2,
    Equals,
    Equals2,
    Extends,
    GetMember,
    GetProperty,
    GetTime,
    GetUrl {
        url: String,
        target: String,
    },
    GetUrl2 {
        send_vars_method: SendVarsMethod,
        is_target_sprite: bool,
        is_load_vars: bool,
    },
    GetVariable,
    GotoFrame(u16),
    GotoFrame2 {
        set_playing: bool,
        scene_offset: u16,
    },
    GotoLabel(String),
    Greater,
    If {
        offset: i16,
    },
    ImplementsOp,
    Increment,
    InitArray,
    InitObject,
    InstanceOf,
    Jump {
        offset: i16,
    },
    Less,
    Less2,
    MBAsciiToChar,
    MBCharToAscii,
    MBStringExtract,
    MBStringLength,
    Modulo,
    Multiply,
    NewMethod,
    NewObject,
    NextFrame,
    Not,
    Or,
    Play,
    Pop,
    PreviousFrame,
    Push(Vec<Value>),
    PushDuplicate,
    RandomNumber,
    RemoveSprite,
    Return,
    SetMember,
    SetProperty,
    SetTarget(String),
    SetTarget2,
    SetVariable,
    StackSwap,
    StartDrag,
    Stop,
    StopSounds,
    StoreRegister(u8),
    StrictEquals,
    StringAdd,
    StringEquals,
    StringExtract,
    StringGreater,
    StringLength,
    StringLess,
    Subtract,
    TargetPath,
    Throw,
    ToInteger,
    ToNumber,
    ToString,
    ToggleQuality,
    Trace,
    Try(TryBlock),
    TypeOf,
    WaitForFrame {
        frame: u16,
        num_actions_to_skip: u8,
    },
    WaitForFrame2 {
        num_actions_to_skip: u8,
    },
    With {
        actions: Vec<Action>,
    },
    Unknown {
        opcode: u8,
        data: Vec<u8>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Undefined,
    Null,
    Bool(bool),
    Int(i32),
    Float(f32),
    Double(f64),
    Str(String),
    Register(u8),
    ConstantPool(u16),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendVarsMethod {
    None,
    Get,
    Post,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<FunctionParam>,
    pub preload_parent: bool,
    pub preload_root: bool,
    pub suppress_super: bool,
    pub preload_super: bool,
    pub suppress_arguments: bool,
    pub preload_arguments: bool,
    pub suppress_this: bool,
    pub preload_this: bool,
    pub preload_global: bool,
    pub actions: Vec<Action>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionParam {
    pub name: String,
    pub register_index: Option<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TryBlock {
    pub try_actions: Vec<Action>,
    pub catch: Option<(CatchVar, Vec<Action>)>,
    pub finally: Option<Vec<Action>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CatchVar {
    Var(String),
    Register(u8),
}

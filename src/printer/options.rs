

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Parens {
    No,
    Allow,
    Yes,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InfixOptions {
    InfixBinary(Parens),
    Long,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InfixChainOptions {
    InfixChain(Parens),
    NoChain(InfixOptions),
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PrintComments {
    Yes,
    No,
}

#[derive(Copy, Clone, Default)]
pub struct FormulaOptions {
    pub not: InfixOptions,
    pub equiv: InfixOptions,
    pub imply: InfixOptions,
    pub or: InfixChainOptions,
    pub and: InfixChainOptions,
    pub odd: InfixChainOptions,
    pub even: InfixOptions,
    pub print_comments: PrintComments,
}


impl Default for Parens {
    fn default() -> Self {
        Parens::Yes
    }
}
impl Default for InfixOptions {
    fn default() -> Self {
        InfixOptions::InfixBinary(Parens::Yes)
    }
}
impl Default for InfixChainOptions {
    fn default() -> Self {
        InfixChainOptions::InfixChain(Parens::Yes)
    }
}
impl Default for PrintComments {
    fn default() -> Self {
        PrintComments::Yes
    }
}

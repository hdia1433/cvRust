use crate::parser::node::VariableDeclaration;

pub struct SemanticScope<'a>
{
    vars: Vec<&'a VariableDeclaration>
}

impl SemanticScope<'_>
{
    pub fn new() -> Self
    {
        Self {vars: Vec::new()}
    }
}

impl<'a> SemanticScope<'a>
{
    pub fn addVar(&mut self, var: &'a VariableDeclaration)
    {
        self.vars.push(var);
    }
}
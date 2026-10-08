use crate::model::nodes::VariableDeclaration;

pub struct SemanticScope
{
    vars: Vec<VariableDeclaration>
}

impl SemanticScope
{
    pub fn new() -> Self
    {
        Self {vars: Vec::new()}
    }

    pub fn addVar(&mut self, var: VariableDeclaration)
    {
        self.vars.push(var);
    }

    pub fn getVars(&self) -> &Vec<VariableDeclaration>
    {
        &self.vars
    }
}
pub use globalScope::GlobalScope;
pub use function::Function;

mod globalScope;
mod function;

pub enum Node
{
    GlobalScope(GlobalScope),
    Function(Function)
}
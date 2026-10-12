use std::fs::File;
use bitvec::prelude::BitVec;

pub trait ToFile
{
    fn getIndent(indents: &BitVec, last: bool) -> String
    {
        let mut indent:String = "\n".to_string();

        for bit in indents
        {
            indent += if *bit
            {
                "│   "
            }
            else
            {
                "    "
            };
        }

        indent += if last
        {
            "└── "
        }
        else
        {
            "├── "
        };

        indent
    }

    fn toFile(&self, f: &mut File, indents: &mut BitVec, last: bool) -> std::io::Result<()>;
}
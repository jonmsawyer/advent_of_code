#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CommandKind {
    None,
    CD,
    LS,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FileType {
    None,
    Dir,
    File,
}

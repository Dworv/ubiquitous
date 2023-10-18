use crate::tools::Tools;

#[derive(Debug, Default)]
pub struct FileSystem {
    pub files: Vec<File>,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub content: FileContent
}

#[derive(Debug)]
pub enum FileContent {
    Text(String),
    Tool(Tools)
}


impl FileContent {
    pub fn get(&self) -> Option<&String> {
        match self {
            FileContent::Text(contents) => Some(contents),
            FileContent::Tool(_) => None,
        }
    }

    pub fn set(&mut self, new_contents: String) -> Result<(), ()> {
        match self {
            FileContent::Text(contents) => {
                *contents = new_contents;
                Ok(())
            },
            FileContent::Tool(_) => Err(()),
        }
    }
}

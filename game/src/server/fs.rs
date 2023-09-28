#[derive(Debug, Default)]
pub struct FileSystem {
    pub files: Vec<File>,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    contents: String,
}

impl File {
    pub fn new(name: String) -> Self {
        File {
            name,
            contents: String::new(),
        }
    }

    pub fn get_contents(&self) -> &String {
        &self.contents // Return self contents
    }

    pub fn set_contents(&mut self, contents: String) {
        self.contents = contents;
    }
}

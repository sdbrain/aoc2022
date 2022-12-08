type FolderLink = Option<Box<Folder>>;

struct Folder {
    size: u64,
    name: String,
    folders: Vec<FolderLink>
}

impl Folder {
    pub fn Folder(name: String) -> Self {
        Folder {
           size: 0,
            name: name,
            folders: Vec::new()
        }
    }

    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(Some(Box::new(folder)));
    }

    pub fn add_size(&mut self, size: u64) {
        self.size += size;
    }
}

pub struct Day6 {}

impl Day6 {

    pub fn new() -> Self {
        Day6 {}
    }

}

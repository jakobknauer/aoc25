pub fn read_input(day: u32) -> InputFile {
    InputFile::new(day)
}

pub struct InputFile {
    contents: String,
}

impl InputFile {
    fn new(day: u32) -> Self {
        let path = format!("input/day{day:02}.txt");
        let contents = std::fs::read_to_string(path).unwrap();
        Self { contents }
    }

    pub fn lines(&self) -> std::str::Lines<'_> {
        self.contents.lines()
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }
}

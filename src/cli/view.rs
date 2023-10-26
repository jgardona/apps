use super::error::Result;

use tabled::{Table, Tabled};

#[derive(Tabled)]
pub struct FileItem {
    name: String,
}

impl FileItem {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub fn display(data: Vec<FileItem>) -> Result<String> {
    Ok(Table::new(data).to_string())
}

#[cfg(test)]
mod view_tests {
    use super::{display, FileItem};

    #[test]
    fn test_display() {
        let data = vec![
            FileItem::new("One".into()),
            FileItem::new("Two".into()),
            FileItem::new("Three".into()),
        ];

        let result = display(data).unwrap();
        let expected = "\
                            +-------+\n\
                            | name  |\n\
                            +-------+\n\
                            | One   |\n\
                            +-------+\n\
                            | Two   |\n\
                            +-------+\n\
                            | Three |\n\
                            +-------+";
        println!("{result}");
        assert_eq!(expected, result);
    }
}

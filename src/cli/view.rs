use super::error::Result;

use tabled::{Table, Tabled};

#[derive(Tabled)]
pub struct FileItem {
    kind: String,
    name: String,
}

impl FileItem {
    pub fn new(kind: String, name: String) -> Self {
        Self { kind, name }
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
            FileItem::new("1".into(), "One".into()),
            FileItem::new("2".into(), "Two".into()),
            FileItem::new("3".into(), "Three".into()),
        ];

        let result = display(data).unwrap();
        let expected = "+------+-------+\n\
                        | kind | name  |\n\
                        +------+-------+\n\
                        | 1    | One   |\n\
                        +------+-------+\n\
                        | 2    | Two   |\n\
                        +------+-------+\n\
                        | 3    | Three |\n\
                        +------+-------+";

        println!("{result}");
        assert_eq!(expected, result);
    }
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct Table {
    data: Vec<Vec<String>>,
    header_map: HashMap<String, usize>,
}

#[derive(Debug)]
pub enum TableError {
    EmptyHeader,
    DuplicateColumn(String),
    RowLengthMismatch {
        row_index: usize,
        row_len: usize,
        header_len: usize,
    },
    InvalidRowIndex(usize),
    InvalidTableSize
}

impl Table {
    /// Creates a new empty table
    pub fn new() -> Self {
        Table {
            data: Vec::new(),
            header_map: HashMap::new(),
        }
    }

    /// Creates a table with header and data
    pub fn with_header_and_data(
        header: Vec<String>,
        data: Vec<Vec<String>>,
    ) -> Result<Self, TableError> {
        if header.is_empty() {
            return Err(TableError::EmptyHeader);
        }

        let mut header_map = HashMap::new();

        for (index, column_name) in header.iter().enumerate() {
            if header_map.insert(column_name.clone(), index).is_some() {
                return Err(TableError::DuplicateColumn(column_name.clone()));
            }
        }

        for (row_index, row) in data.iter().enumerate() {
            if row.len() != header.len() {
                return Err(TableError::RowLengthMismatch {
                    row_index,
                    row_len: row.len(),
                    header_len: header.len(),
                });
            }
        }

        Ok(Table { data, header_map })
    }

    /// Creates a table with only data (no headers)
    pub fn with_data(data: Vec<Vec<String>>) -> Result<Self, TableError> {
        Ok(Table {
            data,
            header_map: HashMap::new(),
        })
    }

    /// Adds a new row to the table
    pub fn add_row(&mut self, row: Vec<String>) -> Result<(), TableError> {
        if !self.header_map.is_empty() && self.header_map.len() != row.len() {
            return Err(TableError::RowLengthMismatch {
                row_index: self.data.len(),
                row_len: row.len(),
                header_len: self.header_map.len(),
            });
        }
        self.data.push(row);
        Ok(())
    }

    /// Gets a row by index
    pub fn get(&self, row_index: usize) -> Option<&Vec<String>> {
        self.data.get(row_index)
    }

    /// Returns the number of rows in the table
    pub fn row_count(&self) -> usize {
        self.data.len()
    }

    /// Returns the number of columns in the table
    pub fn column_count(&self) -> usize {
        self.header_map
            .len()
            .max(self.data.first().map_or(0, |row| row.len()))
    }

    /// Gets a value by row index and column name
    pub fn get_value(&self, row_index: usize, column_name: &str) -> Option<&String> {
        let column_index = self.header_map.get(column_name)?;
        self.data.get(row_index)?.get(*column_index)
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_table() {
        let table = Table::new();
        assert_eq!(table.row_count(), 0);
        assert_eq!(table.column_count(), 0);
    }

    #[test]
    fn test_add_row() {
        let mut table = Table::new();
        let row = vec!["1".to_string(), "2".to_string()];
        assert!(table.add_row(row).is_ok());
    }
}

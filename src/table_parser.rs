use regex::Regex;

use crate::table::{Table, TableError};

#[derive(Debug)]
pub enum TableType {
    AsciiTable,
    CsvTable,
    Unknown,
}

/// Определяет тип таблицы на основе входных данных
/// 
/// # Arguments
/// * `data` - Строка с данными таблицы
/// 
/// # Returns
/// * `TableType` - Определенный тип таблицы
pub fn deduct_table_type(data: &str) -> TableType {
    if data.trim().is_empty() {
        return TableType::Unknown;
    }

    let lines: Vec<&str> = data.lines().collect();

    if lines.len() < 3 {
        if !lines.is_empty() && lines[0].contains(',') {
            return TableType::CsvTable;
        }
        return TableType::Unknown;
    }

    let separator_regex = Regex::new(r"^\+[-]+\+$").unwrap();
    let content_regex = Regex::new(r"^\|.*\|$").unwrap();

    let is_ascii_table = {
        let has_borders = separator_regex.is_match(lines.first().unwrap())
            && separator_regex.is_match(lines.last().unwrap());

        let has_row_separators = lines
            .iter()
            .enumerate()
            .filter(|(index, _)| index % 2 == 1)
            .all(|(_, line)| separator_regex.is_match(line));

        let has_valid_content = lines
            .iter()
            .enumerate()
            .filter(|(index, _)| index % 2 == 0)
            .all(|(_, line)| content_regex.is_match(line));

        has_borders && has_row_separators && has_valid_content
    };

    if is_ascii_table {
        return TableType::AsciiTable;
    }

    let is_csv = {
        let has_commas = lines.iter().all(|line| line.contains(','));

        let first_line_columns = lines
            .first()
            .map(|line| line.matches(',').count() + 1)
            .unwrap_or(0);

        let consistent_columns = lines
            .iter()
            .all(|line| line.matches(',').count() + 1 == first_line_columns);

        has_commas && consistent_columns && first_line_columns > 1
    };

    if is_csv {
        return TableType::CsvTable;
    }

    TableType::Unknown
}

pub fn parse_table(
    table_type: TableType,
    data: &str,
    first_line_is_header: bool,
) -> Result<Table, TableError> {
    match table_type {
        TableType::AsciiTable => parse_ascii_table(data, first_line_is_header),
        TableType::CsvTable => parse_csv_table(data, first_line_is_header),
        TableType::Unknown => Err(TableError::InvalidTableSize),
    }
}

fn parse_csv_table(data: &str, first_line_is_header: bool) -> Result<Table, TableError> {
    let mut lines: Vec<Vec<String>> = data
        .lines()
        .map(|line| line.split(',').map(|s| s.trim().to_string()).collect())
        .collect();

    let result = if first_line_is_header {
        let header = lines.remove(0);
        Table::with_header_and_data(header, lines)?
    } else {
        Table::with_data(lines)?
    };

    Ok(result)
}

fn parse_ascii_table(data: &str, first_line_is_header: bool) -> Result<Table, TableError> {
    let mut lines: Vec<Vec<String>> = data
        .lines()
        .enumerate()
        .filter(|(index, _)| index % 2 == 0)
        .map(|(_, line)| {
            line.split('|')
                .take(line.len() - 1)
                .skip(1)
                .map(|s| s.trim().to_string())
                .collect()
        })
        .collect();

    let result = if first_line_is_header {
        let header = lines.remove(0);
        Table::with_header_and_data(header, lines)?
    } else {
        Table::with_data(lines)?
    };

    Ok(result)
}

/// heuristics to detect if first line is header or not
pub fn first_line_is_header(lines: &Vec<Vec<String>>) -> bool {
    if lines.len() < 2 {
        return false;
    }

    let first_line = &lines[0];
    let second_line = &lines[1];

    if first_line.len() != second_line.len() {
        return false;
    }

    for (header, value) in first_line.iter().zip(second_line.iter()) {
        let second_is_numeric = value.parse::<f64>().is_ok();
        let first_is_numeric = header.parse::<f64>().is_ok();

        if first_is_numeric != second_is_numeric {
            return true;
        }
    }

    first_line.iter().all(|header| {
        header
            .chars()
            .all(|c| c.is_alphabetic() || c.is_whitespace() || c == '_')
            || header.chars().all(|c| c.is_uppercase())
    })
}

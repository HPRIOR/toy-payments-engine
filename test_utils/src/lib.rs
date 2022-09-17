use std::str;

use serde::Serialize;

#[derive(Serialize)]
struct ClientRow {
    client: &'static str,
    available: &'static str,
    held: &'static str,
    total: &'static str,
    locked: &'static str,
}

impl ClientRow {
    fn new(
        client: &'static str,
        available: &'static str,
        held: &'static str,
        total: &'static str,
        locked: &'static str,
    ) -> Self {
        Self {
            client,
            available,
            held,
            total,
            locked,
        }
    }
}

// Only used during testing so no need to return result
pub fn create_csv(rows: Vec<[&'static str; 5]>) -> String{
    let client_rows: Vec<ClientRow> = rows
        .into_iter()
        .map(|r| ClientRow::new(r[0], r[1], r[2], r[3], r[4]))
        .collect();


    let mut wtr = csv::Writer::from_writer(vec![]);
    for c in client_rows {
        wtr.serialize(c).unwrap();
    }
    wtr.flush().unwrap();
    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    data
}

#[cfg(test)]
mod tests {
    use crate::create_csv;

    #[test]
    fn creates_single_row() {
        let rows = vec![["1", "2", "3", "4", "5"]];
        let sut = create_csv(rows);
        let expected = String::from("client,available,held,total,locked\n1,2,3,4,5\n");
        assert_eq!(sut, expected);
    }

    #[test]
    fn create_multiple_rows() {
        let rows = vec![["1", "2", "3", "4", "5"], ["1", "2", "3", "4", "5"]];
        let sut = create_csv(rows);
        let expected = String::from("client,available,held,total,locked\n1,2,3,4,5\n1,2,3,4,5\n");
        assert_eq!(sut, expected);
    }
}


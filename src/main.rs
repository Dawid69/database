mod db;
use crate::db::DBValue;
use crate::db::Database;

const DATABASE: &str = "db.json";

fn main() {
    let mut db: Database = Database::new();

    db.open(DATABASE);

    db.insert(DBValue {
        entry: format!("TestValue"),
    });

    if let Err(e) = db.delete(3) {
        println!("Cannot delete entry because: {:?}", e)
    }

    if db.search_by_key(4).is_err() {
        println!("This Value does not exist!!: {}", 4);
    } else {
        println!("Search by key works")
    }

    if db
        .search_by_val(DBValue {
            entry: "TestValue9".into(),
        })
        .is_err()
    {
        println!("This Entry does not exist!!: {}", "TestValue9");
    } else {
        println!("Search by val works")
    }

    if let Err(e) = db.update(
        4,
        DBValue {
            entry: "HELLOOOO".into(),
        },
    ) {
        println!("Cannot update because of : {:?}", e)
    }

    db.save_db(DATABASE);
    println!("{:?}", db);
}

extern crate r2d2;
#[macro_use] extern crate nickel;
extern crate nickel_sqlite;
extern crate r2d2_sqlite;

use r2d2::{Pool, Config};
use r2d2_sqlite::SqliteConnectionManager;
use nickel::{Nickel, HttpRouter};
use nickel_sqlite::{SqliteMiddleware, SqliteRequestExtensions};

#[derive(Debug)]
struct Person {
  id   : i32,
  name : String
}

const CREATE_TABLE : &'static str = "
  CREATE TABLE person (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL
  )";

fn main() {

  let mut app = Nickel::new();
  let db_url  = "file.db";
  let db_mgr  = SqliteConnectionManager::new(db_url.as_ref())
                  .expect("Unable to connect to database");
  let db_pool = Pool::new(Config::default(), db_mgr)
                  .expect("Unable to initialize connection pool");
  let db      = db_pool.clone().get().unwrap();

  match db.execute(CREATE_TABLE, &[]) {
    Ok(_)  => println!("created table 'person'"),
    Err(_) => {} // handle err
  };

  app.utilize(SqliteMiddleware::with_pool(db_pool));

  app.get("/persons/new/:name", middleware! { |req|
    let name = req.param("name").unwrap();
    let db = req.db_conn().unwrap();
    match db.execute("INSERT INTO person (name) VALUES ($1)", &[&name]) {
      Ok(_)    => format!("Sucessfully created an entry"),
      Err(err) => format!("Could not create a new entry: {}", err)
    }
  });

  app.get("/persons", middleware! { |request|
    let db = request.db_conn().unwrap();
    let mut stmt = db.prepare("SELECT id, name FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
      Person{
        id   : row.get(0),
        name : row.get(1)
      }
    }).unwrap();
    let list = person_iter
        .map(|x| format!("<li>{}</li>", x.unwrap().name))
        .collect::<Vec<String>>()
        .concat();
    format!("<html><ul>{}</ul></html>", list)
  });

  app.listen("127.0.0.1:6767");
}

// Copyright (c) 2015 - 2016 nickel-sqlite developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [nickel-sqlite](https://github.com/flosse/nickel-sqlite) is a
//! [SQLite](http://www.sqlite.org/) middleware for
//! [nickel.rs](http://nickel.rs/).
//!
//! # Example
//!
//! ```no_run
//! extern crate r2d2;
//! #[macro_use] extern crate nickel;
//! extern crate nickel_sqlite;
//! extern crate r2d2_sqlite;
//!
//! use r2d2::Pool;
//! use r2d2_sqlite::SqliteConnectionManager;
//! use nickel::{Nickel, HttpRouter};
//! use nickel_sqlite::{SqliteMiddleware, SqliteRequestExtensions};
//!
//! #[derive(Debug)]
//! struct Person {
//!   id   : i32,
//!   name : String
//! }
//!
//! const CREATE_TABLE : &'static str = "
//!   CREATE TABLE person (
//!     id   INTEGER PRIMARY KEY,
//!     name TEXT NOT NULL
//!   )";
//!
//! fn main() {
//!
//!   let mut app = Nickel::new();
//!   let db_url  = "file.db";
//!   let mw      = SqliteMiddleware::new(&db_url).expect("Unable to connect to database");
//!   let db      = mw.pool.clone().get().unwrap();
//!
//!   match db.execute(CREATE_TABLE, &[]) {
//!     Ok(_)  => println!("created table 'person'"),
//!     Err(_) => {} // handle error
//!   };
//!
//!   app.utilize(mw);
//!
//!   app.get("/persons/new/:name", middleware! { |req|
//!     let name = req.param("name").unwrap();
//!     let db = req.db_conn().unwrap();
//!     match db.execute("INSERT INTO person (name) VALUES ($1)", &[&name]) {
//!       Ok(_)    => format!("Sucessfully created an entry"),
//!       Err(err) => format!("Could not create a new entry: {}", err)
//!     }
//!   });
//!
//!   app.get("/persons", middleware! { |request|
//!     let db = request.db_conn().unwrap();
//!     let mut stmt = db.prepare("SELECT id, name FROM person").unwrap();
//!     let person_iter = stmt.query_map(&[], |row| {
//!       Person{
//!         id   : row.get(0),
//!         name : row.get(1)
//!       }
//!     }).unwrap();
//!     let list = person_iter
//!         .map(|x| format!("<li>{}</li>", x.unwrap().name))
//!         .collect::<Vec<String>>()
//!         .concat();
//!     format!("<html><ul>{}</ul></html>", list)
//!   });
//!
//!   app.listen("127.0.0.1:6767");
//! }
//! ```

extern crate nickel;
extern crate plugin;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate typemap;

pub use middleware::{SqliteMiddleware, SqliteRequestExtensions};

mod middleware;

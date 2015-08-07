extern crate nickel;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate plugin;
extern crate typemap;

pub use middleware::{ SqliteMiddleware, SqliteRequestExtensions };

mod middleware;

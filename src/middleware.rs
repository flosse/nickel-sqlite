use std::sync::Arc;
use std::error::Error as StdError;

use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use r2d2_sqlite::{SqliteConnectionManager};
use r2d2::{Pool, HandleError, Config, PooledConnection};
use typemap::Key;
use plugin::{Pluggable, Extensible};

pub struct SqliteMiddleware {
  pub pool: Arc<Pool<SqliteConnectionManager>>
}

impl SqliteMiddleware {
  pub fn new(connect_str: &str,
             num_connections: u32,
             error_handler: Box<HandleError<::r2d2_sqlite::Error>>)
               -> Result<SqliteMiddleware, Box<StdError>> {
      let config = Config::builder()
        .pool_size(num_connections)
        .error_handler(error_handler)
        .build();
      let manager = try!(SqliteConnectionManager::new(connect_str));
      let pool = try!(Pool::new(config, manager));
      Ok(SqliteMiddleware { pool: Arc::new(pool) })
  }
}

impl Key for SqliteMiddleware { type Value = Arc<Pool<SqliteConnectionManager>>; }

impl<D> Middleware<D> for SqliteMiddleware {
  fn invoke<'a>(&self, req: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
    req.extensions_mut().insert::<SqliteMiddleware>(self.pool.clone());
    Ok(Continue(res))
  }
}

pub trait SqliteRequestExtensions {
  fn db_conn(&self) -> PooledConnection<SqliteConnectionManager>;
}

impl<'a, 'b> SqliteRequestExtensions for Request<'a, 'b> {
  fn db_conn(&self) -> PooledConnection<SqliteConnectionManager> {
    self.extensions().get::<SqliteMiddleware>().unwrap().get().unwrap()
  }
}

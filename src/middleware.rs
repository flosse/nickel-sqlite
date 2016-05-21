use std::result::Result;
use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::{Pool, PooledConnection, GetTimeout};
use typemap::Key;
use plugin::Extensible;

pub struct SqliteMiddleware {
  pub pool: Pool<SqliteConnectionManager>
}

impl SqliteMiddleware {
  pub fn new(pool: Pool<SqliteConnectionManager>) -> SqliteMiddleware {
    SqliteMiddleware { pool: pool }
  }
}

impl Key for SqliteMiddleware { type Value = Pool<SqliteConnectionManager>; }

impl<D> Middleware<D> for SqliteMiddleware {
  fn invoke<'a>(&self, req: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
    req.extensions_mut().insert::<SqliteMiddleware>(self.pool.clone());
    Ok(Continue(res))
  }
}

pub trait SqliteRequestExtensions {
  fn db_conn(&self) -> Result<PooledConnection<SqliteConnectionManager>, GetTimeout>;
}

impl<'a, 'b, D> SqliteRequestExtensions for Request<'a, 'b, D> {
  fn db_conn(&self) -> Result<PooledConnection<SqliteConnectionManager>, GetTimeout> {
    self.extensions().get::<SqliteMiddleware>().unwrap().get()
  }
}

use std::error::Error;
use std::result::Result;
use nickel::{Continue, Middleware, MiddlewareResult, Request, Response};
use nickel::status::StatusCode;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::{self, Pool, PooledConnection};
use typemap::Key;
use plugin::Extensible;

pub struct SqliteMiddleware {
    pub pool: Pool<SqliteConnectionManager>,
}

impl SqliteMiddleware {
    /// Create middleware using defaults
    ///
    /// The middleware will be setup with the r2d2 defaults.
    pub fn new(db_url: &str) -> Result<SqliteMiddleware, Box<Error>> {
        let manager = SqliteConnectionManager::file(db_url);
        let pool = Pool::builder().build(manager)?;
        Ok(SqliteMiddleware { pool })
    }

    /// Create middleware using pre-built `r2d2::Pool`
    ///
    /// This allows the caller to create and configure the pool with specific settings.
    pub fn with_pool(pool: Pool<SqliteConnectionManager>) -> SqliteMiddleware {
        SqliteMiddleware { pool: pool }
    }
}

impl Key for SqliteMiddleware {
    type Value = Pool<SqliteConnectionManager>;
}

impl<D> Middleware<D> for SqliteMiddleware {
    fn invoke<'a>(&self, req: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
        req.extensions_mut()
            .insert::<SqliteMiddleware>(self.pool.clone());
        Ok(Continue(res))
    }
}

/// Add `db_conn()` helper method to `nickel::Request`
///
/// This trait must only be used in conjunction with `SqliteMiddleware`.
///
/// On error, the method returns a tuple per Nickel convention.
/// This allows the route to use the `try_with!` macro.
///
/// Example:
///
/// ```ignore
/// app.get("/my_counter", middleware! { |request, response|
/// 	let db = try_with!(response, request.db_conn());
/// });
/// ```
pub trait SqliteRequestExtensions {
    fn db_conn(
        &self,
    ) -> Result<PooledConnection<SqliteConnectionManager>, (StatusCode, r2d2::Error)>;
}

impl<'a, 'b, D> SqliteRequestExtensions for Request<'a, 'b, D> {
    fn db_conn(
        &self,
    ) -> Result<PooledConnection<SqliteConnectionManager>, (StatusCode, r2d2::Error)> {
        self.extensions()
            .get::<SqliteMiddleware>()
            .expect("SqliteMiddleware must be registered before using SqliteRequestExtensions")
            .get()
            .or_else(|err| Err((StatusCode::InternalServerError, err)))
    }
}

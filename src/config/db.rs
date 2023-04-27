use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

/// Connection type for postgres
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

/// Connection pool type for postgres
pub type PoolConnection = Pool<ConnectionManager<PgConnection>>;

/// Function to establish connection with postgres
///
/// # Arguments
///
/// * `db_url` - A string slice containing the database connection string
///
/// # Returns
///
/// * A connection pool or an error message
pub fn establish_connection(db_url: &str) -> PoolConnection {
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    return Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
}

/// Function to get connection from pool
///
/// # Arguments
///
/// * `pool` - A pool of database connections
///
/// # Returns
///
/// * A connection from the pool or an error message
pub fn get_connection(pool: &Pool<ConnectionManager<PgConnection>>) -> Result<Connection, String> {
    let db_connection = pool.get();

    match db_connection {
        Ok(conn) => return Ok(conn),
        Err(_) => return Err("Failed to get connection".to_string()),
    }
}

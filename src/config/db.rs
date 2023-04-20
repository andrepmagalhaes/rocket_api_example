use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;
pub type PoolConnection = Pool<ConnectionManager<PgConnection>>; 

pub fn establish_connection(db_url: &str) -> PoolConnection {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    
    return Pool::builder().build(manager).expect("Failed to create pool.");   
}

pub fn get_connection(pool: &Pool<ConnectionManager<PgConnection>>) -> Result<Connection, String> {
    let db_connection = pool.get();

    match db_connection  {
        Ok(conn) => return Ok(conn),
        Err(_) => return Err("Failed to get connection".to_string()),
    }

}
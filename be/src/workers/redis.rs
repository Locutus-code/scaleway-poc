use crate::models::state::ApplicationState;

use redis::{ConnectionAddr, ConnectionInfo, RedisConnectionInfo};

fn get_conninfo(state: &ApplicationState) -> ConnectionInfo {
    ConnectionInfo {
        addr: ConnectionAddr::TcpTls {
            host: state.redis_host.clone(),
            port: state.redis_port.clone(),
            insecure: true,
            tls_params: None,
        },
        redis: RedisConnectionInfo {
            db: 0,
            username: Some(state.redis_username.clone()),
            password: Some(state.redis_password.clone()),
            protocol: redis::ProtocolVersion::RESP2,
        },
    }
}

pub async fn get_conn(state: &ApplicationState) -> redis::aio::MultiplexedConnection {
    let conninfo = get_conninfo(state);
    let client = redis::Client::open(conninfo).expect("Error starting REDIS client");
    let mut conn = client.get_multiplexed_async_connection().await;

    let retval = match conn {
        Ok(conn) => conn,
        Err(e) => panic!("Opening REDIS connection failed {:?}", e),
    };
    return retval;
}

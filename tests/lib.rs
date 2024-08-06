use redis::{Commands, Connection};
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::SyncRunner,
    Container, GenericImage,
};

fn new_container() -> Result<GenericImage, Box<dyn std::error::Error + 'static>> {
    let container = GenericImage::new("redis_lingua", "latest")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"));

    Ok(container)
}

fn new_connection(
    container: &Container<GenericImage>,
) -> Result<Connection, Box<dyn std::error::Error + 'static>> {
    let host = container.get_host()?;
    let host_port = container.get_host_port_ipv4(6379)?;
    let url = format!("redis://{}:{}", host, host_port);

    let client = redis::Client::open(url)?;
    let connection = client.get_connection()?;

    Ok(connection)
}

#[test]
fn lingua_detect() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let container = new_container()?.start()?;
    let mut connection = new_connection(&container)?;

    let language: String = redis::cmd("LINGUA.DETECT")
        .arg("buongiorno")
        .query(&mut connection)?;

    assert_eq!(language, "Italian");

    Ok(())
}

#[test]
fn lingua_get() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let container = new_container()?.start()?;
    let mut connection = new_connection(&container)?;

    connection.set("italian", "buongiorno")?;

    let language: String = redis::cmd("LINGUA.GET")
        .arg("italian")
        .query(&mut connection)?;

    assert_eq!(language, "Italian");

    Ok(())
}

#[test]
fn lingua_hget() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let container = new_container()?.start()?;
    let mut connection = new_connection(&container)?;

    connection.hset("italian", "greeting", "buongiorno")?;

    let language: String = redis::cmd("LINGUA.HGET")
        .arg("italian")
        .arg("greeting")
        .query(&mut connection)?;

    assert_eq!(language, "Italian");

    Ok(())
}

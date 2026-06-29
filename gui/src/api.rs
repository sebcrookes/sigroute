
use zbus::Connection;
use zbus::proxy;

#[proxy(
    interface = "uk.co.sebcrookes.Sigroute",
    default_service = "uk.co.sebcrookes.Sigroute",
    default_path = "/uk/co/sebcrookes/Sigroute",
    gen_blocking = true
)]
trait AutomationAPI {
    fn get_version(&self) -> zbus::Result<String>;
}

pub struct APIConnection {
    connection: Connection,
    proxy: AutomationAPIProxy<'static>,
}

pub async fn open_connection() -> zbus::Result<APIConnection> {
    let connection = Connection::session().await?;
    let proxy = AutomationAPIProxy::new(&connection).await?;

    Ok(APIConnection { connection: connection, proxy: proxy })
}

pub async fn get_version(conn: APIConnection) -> zbus::Result<String> {
    return conn.proxy.get_version().await;
}

use sigroute_common::Automation;
use sigroute_common::AutomationAction;
use sigroute_common::AutomationTrigger;
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
    fn get_automations(&self) -> zbus::Result<Vec<Automation>>;
    fn get_automation_triggers(&self, automation_id: i64) -> zbus::Result<Vec<AutomationTrigger>>;
    fn get_automation_actions(&self, automation_id: i64) -> zbus::Result<Vec<AutomationAction>>;
}

pub struct APIConnection {
    _connection: Connection,
    proxy: AutomationAPIProxy<'static>,
}

pub async fn open_connection() -> zbus::Result<APIConnection> {
    let connection = Connection::session().await?;
    let proxy = AutomationAPIProxy::new(&connection).await?;

    Ok(APIConnection { _connection: connection, proxy: proxy })
}

pub async fn get_version(conn: &APIConnection) -> zbus::Result<String> {
    return conn.proxy.get_version().await;
}

pub async fn get_automations(conn: &APIConnection) -> zbus::Result<Vec<Automation>> {
    return conn.proxy.get_automations().await;
}

pub async fn get_automation_triggers(conn: &APIConnection, automation_id: i64) -> zbus::Result<Vec<AutomationTrigger>> {
    return conn.proxy.get_automation_triggers(automation_id).await;
}

pub async fn get_automation_actions(conn: &APIConnection, automation_id: i64) -> zbus::Result<Vec<AutomationAction>> {
    return conn.proxy.get_automation_actions(automation_id).await;
}

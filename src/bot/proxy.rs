use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "me.endercass.skiddo_manager.SkiddoManagerInterface",
    default_service = "me.endercass.skiddo_manager",
    default_path = "/me/endercass/skiddo_manager"
)]
trait SkiddoManagerInterface {
    async fn spawn(&mut self, skiddo_file: String) -> zbus::Result<()>;
    async fn eval(&self, skiddo_file: String, code: String) -> zbus::Result<String>;
    async fn instances(&self) -> zbus::Result<Vec<String>>;
}

pub trait PacketHandler {
    fn handle(client: &crate::Client, data: serde_json::Value) -> crate::Result<()>;
}
pub enum GatewayOp {
    Dispatch,
    Heartbeat,
    Identify,
    PresenceUpdate,
    VoiceStateUpdate,
    Resume,
    Reconnect,
    ReqGuildMember,
    InvalidSession,
    Hello,
    HeartbeatRes
}

use self::GatewayOp::*;

impl GatewayOp {
    pub fn code(&self) -> u8 {
        match self {
            Dispatch => 0,
            Heartbeat => 1,
            Identify => 2,
            PresenceUpdate => 3,
            VoiceStateUpdate => 4,
            Resume => 6,
            Reconnect => 7,
            ReqGuildMember => 8,
            InvalidSession => 9,
            Hello => 10,
            HeartbeatRes => 11
        }
    }

    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Dispatch),
            1 => Some(Heartbeat),
            2 => Some(Identify),
            3 => Some(PresenceUpdate),
            4 => Some(VoiceStateUpdate),
            6 => Some(Resume),
            7 => Some(Reconnect),
            8 => Some(ReqGuildMember),
            9 => Some(InvalidSession),
            10 => Some(Hello),
            11 => Some(HeartbeatRes),
            _ => None
        }
    }
}
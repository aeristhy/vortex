use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoStaticStr;

use mediasoup::rtp_parameters::{MediaKind, RtpCapabilitiesFinalized, RtpParameters};

use crate::rtc::types::{ConnectTransportData, InitializationInput, TransportInitData};
use crate::state::user::{ProduceType, UserInfo};

#[derive(Deserialize, IntoStaticStr)]
#[serde(tag = "type", content = "data")]
pub enum WSCommandType {
    #[serde(rename_all = "camelCase")]
    Authenticate {
        room_id: String,
        token: String,
    },

    InitializeTransports {
        #[serde(flatten)]
        init_data: InitializationInput,
    },
    ConnectTransport {
        #[serde(flatten)]
        connect_data: ConnectTransportData,
    },

    RoomInfo,

    #[serde(rename_all = "camelCase")]
    StartProduce {
        produce_type: ProduceType,
        rtp_parameters: RtpParameters,
    },
    #[serde(rename_all = "camelCase")]
    StopProduce {
        produce_type: ProduceType,
    },

    #[serde(rename_all = "camelCase")]
    StartConsume {
        produce_type: ProduceType,
        user_id: String,
    },
    StopConsume {
        /// Consumer ID
        id: String,
    },
    SetConsumerPause {
        /// Consumer ID
        id: String,
        paused: bool,
    },
}

#[derive(Deserialize)]
pub struct WSCommand {
    pub id: Option<String>,
    #[serde(flatten)]
    pub command_type: WSCommandType,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "camelCase")]
pub enum WSReplyType {
    #[serde(rename_all = "camelCase")]
    Authenticate {
        user_id: String,
        room_id: String,
        rtp_capabilities: RtpCapabilitiesFinalized,
    },

    InitializeTransports {
        #[serde(flatten)]
        reply_data: TransportInitData,
    },
    ConnectTransport,

    #[serde(rename_all = "camelCase")]
    RoomInfo {
        id: String,
        video_allowed: bool,
        users: HashMap<String, UserInfo>,
    },

    #[serde(rename_all = "camelCase")]
    StartProduce {
        producer_id: String,
    },
    StopProduce,

    #[serde(rename_all = "camelCase")]
    StartConsume {
        id: String,
        producer_id: String,
        kind: MediaKind,
        rtp_parameters: RtpParameters,
    },
    StopConsume,
    SetConsumerPause,
}

#[derive(Serialize)]
pub struct WSReply {
    pub id: Option<String>,
    #[serde(flatten)]
    pub reply_type: WSReplyType,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "camelCase")]
pub enum WSEvent {
    UserJoined {
        id: String,
    },
    UserLeft {
        id: String,
    },

    UserStartProduce {
        id: String,
        #[serde(rename = "type")]
        produce_type: ProduceType,
    },
    UserStopProduce {
        id: String,
        #[serde(rename = "type")]
        produce_type: ProduceType,
    },
}

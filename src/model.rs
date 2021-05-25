use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize)]
pub struct Interaction {
    #[serde(rename = "type")]
    pub interaction_type: InteractionType,
    data: Option<ApplicationCommandInteractionData>,
    guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    member: Option<GuildMember>,
    pub token: String,
    version: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuildMember {
    pub deaf: bool,
    pub nick: Option<String>,
    pub roles: Vec<String>,
    /// Attached User struct.
    pub user: User,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Snowflake,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub discriminator: String,
    pub username: String,
}
#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

type Snowflake = String;
#[derive(Debug, Deserialize)]
struct ApplicationCommandInteractionData {
    id: Snowflake,
    name: String,
    //resolved: Option<ApplicationCommandInteractionDataResolved>
    options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}

#[derive(Debug, Deserialize)]
struct ApplicationCommandInteractionDataOption {
    name: String,
    #[serde(rename = "type")]
    option_type: ApplicationCommandOptionType,
    // the value of the pair
    value: Option<String>,
    // present if this option is a group or subcommand
    options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}
#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
enum ApplicationCommandOptionType {
    SUBCOMMAND = 1,
    SUBCOMMANDGROUP = 2,
    STRING = 3,
    INTEGER = 4,
    BOOLEAN = 5,
    USER = 6,
    CHANNEL = 7,
    ROLE = 8,
}

#[derive(Serialize, Debug)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub interaction_response_type: InteractionResponseType,
    pub data: Option<InteractionApplicationCommandCallbackData>,
}

impl InteractionResponse {
    pub fn reply(content: String) -> InteractionResponse {
        InteractionResponse {
            interaction_response_type: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionApplicationCommandCallbackData {
                tts: None,
                content: Some(content),
                flags: None,
            }),
        }
    }
    pub fn into_response(self) -> Response<Body> {
        Response::builder()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(
                serde_json::to_string(&self)
                    .expect("unable to serialize serde_json::Value")
                    .into(),
            )
            .expect("unable to build http::Response")
    }
}

#[derive(Serialize_repr, Debug)]
#[repr(u8)]
pub enum InteractionResponseType {
    Pong = 1,
    // Acknowledge = 2,
    // ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    ACKWithSource = 5,
}

#[derive(Serialize, Debug)]
pub struct InteractionApplicationCommandCallbackData {
    pub tts: Option<bool>,
    pub content: Option<String>,
    // embeds
    // allowed_mentions
    pub flags: Option<usize>,
}

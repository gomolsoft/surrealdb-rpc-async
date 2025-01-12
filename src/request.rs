use rand::{thread_rng, Rng};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub(crate) struct SurrealRequest {
    id: String,
    #[serde(flatten)]
    payload: SurrealRequestPayload,
}
impl From<SurrealRequestPayload> for SurrealRequest {
    fn from(payload: SurrealRequestPayload) -> SurrealRequest {
        SurrealRequest {
            id: base64::encode(&thread_rng().gen::<u128>().to_le_bytes()),
            payload,
        }
    }
}
impl SurrealRequest {
    pub(crate) fn id(&self) -> &str {
        &self.id
    }
    pub(crate) fn ping() -> SurrealRequest {
        SurrealRequestPayload::Ping([]).into()
    }
    pub(crate) fn use_ns_db(ns: String, db: String) -> SurrealRequest {
        SurrealRequestPayload::Use(ns, db).into()
    }
    pub(crate) fn query(query: String, params: BTreeMap<String, Value>) -> SurrealRequest {
        SurrealRequestPayload::Query(query, params).into()
    }
    pub(crate) fn sign_in(username: String, password: String) -> SurrealRequest {
        SurrealRequestPayload::SignIn((SignInParams {
            user: username,
            pass: password,
        },))
        .into()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
#[serde(tag = "method", content = "params", rename_all = "lowercase")]
pub(crate) enum SurrealRequestPayload {
    Ping([(); 0]),
    Use(String, String),
    Query(String, BTreeMap<String, Value>),
    SignIn((SignInParams,)),
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub(crate) struct SignInParams {
    user: String,
    pass: String,
}

#[test]
fn check_se() {
    let x = SurrealRequest {
        id: "abc".to_string(),
        payload: SurrealRequestPayload::Query("CREATE user SET name = $name".to_string(), {
            let mut map = BTreeMap::new();
            map.insert("user".into(), Value::String("Lucy".into()));
            map
        }),
    };
    let s = serde_json::to_string(&x).unwrap();
    assert_eq!(
        s,
        r#"{"id":"abc","method":"query","params":["CREATE user SET name = $name",{"user":"Lucy"}]}"#
    );
}

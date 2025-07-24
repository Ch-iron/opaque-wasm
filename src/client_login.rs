use crate::hash_methods::Default;
use opaque_ke::{
    ClientLogin, ClientLoginFinishParameters, CredentialResponse,
};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Login {
    state: Option<ClientLogin<Default>>,
    rng: OsRng,
    session_key: Option<Vec<u8>>,
    export_key: Option<Vec<u8>>,
    server_s_pk: Option<Vec<u8>>,
}

#[wasm_bindgen]
impl Login {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Login {
        Login {
            rng: OsRng,
            state: None,
            session_key: None,
            export_key: None,
            server_s_pk: None,
        }
    }

    pub fn start(&mut self, password: &str) -> Result<Vec<u8>, JsValue> {
        let client_login_start_result = match ClientLogin::<Default>::start(
            &mut self.rng,
            &password.as_bytes()
        ) {
            Ok(client_login_start_result) => client_login_start_result,
            Err(_e) => return Err("Failed start".into()),
        };

        self.state = Some(client_login_start_result.state);

        return Ok(client_login_start_result.message.serialize().to_vec());
    }

    pub fn finish(&mut self, pass: &str, message: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let message = CredentialResponse::deserialize(&message[..]);

        if message.is_err() {
            return Err("Message deserialize failed".into());
        }

        let state = self.state.take();

        let result = state
            .unwrap()
            .finish(pass.as_bytes(), message.unwrap(), ClientLoginFinishParameters::default())
            .unwrap();

        self.session_key = Some(result.session_key.to_vec());
        self.export_key = Some(result.export_key.to_vec());
        
        let server_s_pk_bytes = result.server_s_pk.serialize();
        self.server_s_pk = Some(server_s_pk_bytes.to_vec());

        return Ok(result.message.serialize().to_vec());
    }

    #[wasm_bindgen(js_name = getSessionKey)]
    pub fn get_session_key(&self) -> Result<Vec<u8>, JsValue> {
        return Ok(self.session_key.to_owned().unwrap().to_vec());
    }

    #[wasm_bindgen(js_name = getExportKey)]
    pub fn get_export_key(&self) -> Result<Vec<u8>, JsValue> {
        return Ok(self.export_key.to_owned().unwrap());
    }

    #[wasm_bindgen(js_name = getServerSPK)]
    pub fn get_server_s_pk(&self) -> Result<Vec<u8>, JsValue> {
        return Ok(self.server_s_pk.to_owned().unwrap());
    }
}

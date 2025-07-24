use crate::hash_methods::Default;
use opaque_ke::{ClientRegistration, ClientRegistrationFinishParameters, RegistrationResponse};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Registration {
    state: Option<ClientRegistration<Default>>,
    rng: OsRng,
    export_key: Option<Vec<u8>>,
    server_s_pk: Option<Vec<u8>>,
}

#[wasm_bindgen]
impl Registration {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Registration {
        Registration {
            rng: OsRng,
            state: None,
            export_key: None,
            server_s_pk: None,
        }
    }

    pub fn start(&mut self, password: &str) -> Result<Vec<u8>, JsValue> {
        let client_registration_start_result =
            match ClientRegistration::<Default>::start(&mut self.rng, &password.as_bytes()) {
                Ok(reply) => reply,
                Err(_e) => return Err("Start failed".into()),
            };
        self.state = Some(client_registration_start_result.state);

        return Ok(client_registration_start_result.message.serialize().to_vec());
    }

    pub fn finish(&mut self, pass: &str, message: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let message = match RegistrationResponse::deserialize(&message[..]) {
            Ok(message) => message,
            Err(_e) => return Err("Message deserialize failed".into()),
        };
        let mut rng = self.rng;

        let state = self.state.take();

        let client_finish_registration_result = match state.unwrap().finish(
            &mut rng,
            pass.as_bytes(),
            message,
            ClientRegistrationFinishParameters::default(),
        ) {
            Ok(reply) => reply,
            Err(_e) => return Err("Mismatch messagess".into()),
        };

        self.export_key = Some(client_finish_registration_result.export_key.to_vec());
        
        let server_s_pk_bytes = client_finish_registration_result.server_s_pk.serialize();
        self.server_s_pk = Some(server_s_pk_bytes.to_vec());

        return Ok(client_finish_registration_result.message.serialize().to_vec());
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

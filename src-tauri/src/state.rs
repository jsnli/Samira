use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{AppHandle, State, Manager};
use steamworks::{Client, ClientManager};

pub struct AppState {
    pub db: Mutex<Option<Connection>>,
    pub client: Mutex<Option<Client<ClientManager>>>,
}

pub trait ServiceAccess {
    // fn db<F, TResult>(&self, operation: F) -> TResult
    // where
    //     F: FnOnce(&Connection) -> TResult;
    //
    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult;
}

impl ServiceAccess for AppHandle {
    // fn db<F, TResult>(&self, operation: F) -> TResult
    // where
    //     F: FnOnce(&Connection) -> TResult,
    // {
    //     let app_state: State<AppState> = self.state();
    //     let db_connection_guard = app_state.db.lock().unwrap();
    //     let db = db_connection_guard.as_ref().unwrap();
    //
    //     operation(db)
    // }

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let mut db_connection_guard = app_state.db.lock().unwrap();
        let db = db_connection_guard.as_mut().unwrap();

        operation(db)
    }
}

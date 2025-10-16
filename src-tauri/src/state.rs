// use rusqlite::Connection;
//
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

use crate::dataset::Game;
use steamworks::Client;

pub struct AppState {
    pub data: Mutex<Option<Vec<Game>>>,
    pub client: Mutex<Option<Client>>,
}

pub trait ServiceAccess {
    fn data<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Vec<Game>) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn data<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Vec<Game>) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let data_guard = app_state.data.lock().unwrap();
        let data = data_guard
            .as_ref()
            .expect("Dataset not loaded yet");

        operation(data)
    }
}
/*
pub struct AppState {
    pub db: Mutex<Option<Connection>>,
    pub client: Mutex<Option<Client>>,
}

pub trait ServiceAccess {
    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult;
}

impl ServiceAccess for AppHandle {
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
*/

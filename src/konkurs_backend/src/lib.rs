use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::cell::RefCell;
use ic_cdk_macros::{init, query, update};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct User {
    username: String,
    password: String,
}

thread_local! {
    static UZYTKOWNICY: RefCell<Vec<User>> = RefCell::new(Vec::new());
}

#[init]
fn init() {
    UZYTKOWNICY.with(|user_storage| {
        *user_storage.borrow_mut() = Vec::new();
    });
}

#[update]
fn dodaj_uzytkownika(username: String, password: String) -> String {
    UZYTKOWNICY.with(|user_storage| {
        let mut users = user_storage.borrow_mut();
        if users.iter().any(|user| user.username == username) {
            return "Użytkownik o tej nazwie już istnieje.".to_string();
        }
        users.push(User { username, password });
        "Użytkownik dodany pomyślnie.".to_string()
    })
}

#[query]
fn zaloguj(username: String, password: String) -> String {
    let mut attempted_users = String::new();
    UZYTKOWNICY.with(|user_storage| {
        let users = user_storage.borrow();
        for user in users.iter() {
            if user.username.to_lowercase() == username.to_lowercase() && user.password == password {
                return format!("Zalogowano jako {}.", username);
            } else {
                if !attempted_users.is_empty() {
                    attempted_users.push_str(" ");
                }
                attempted_users.push_str(&format!("User: {}, ", user.username));
            }
        }
        
        if !attempted_users.is_empty() {
            format!("Logowanie nieudane. Spróbowano: {}", attempted_users)
        } else {
            "Logowanie nieudane.".to_string()
        }
    })

    
}
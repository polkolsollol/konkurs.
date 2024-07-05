use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::{cell::RefCell, clone};
use ic_cdk_macros::{init, query, update};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct User {
    username: String,
    password: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct Awaria {
    nazwa: String,
    komentarze: Vec<String>,
}

thread_local! {
    static UZYTKOWNICY: RefCell<Vec<User>> = RefCell::new(Vec::new());
    static AWARIE: RefCell<Vec<Awaria>> = RefCell::new(Vec::new());
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
fn zaloguj(username: String, password: String) -> bool {
    UZYTKOWNICY.with(|user_storage| {
        let users = user_storage.borrow();
        for user in users.iter() {
            if user.username.to_lowercase() == username.to_lowercase() && user.password == password {
                return true;
            }
        }
        return false;
    })
}


#[update]
fn dodaj_awarie(nazwa: String) {
    AWARIE.with(|awarie| {
        awarie.borrow_mut().push(Awaria {
            nazwa,
            komentarze: vec![],
        })
    })
} 

#[query]
fn odczytaj_awarie() -> Vec<Awaria> {
    AWARIE.with(|awarie_storage| {
        let awarie = awarie_storage.borrow().clone();
        return awarie;
    })
}

#[update]
fn usun_awarie(id_awarii: usize) {
    AWARIE.with(|awarie| {
        let mut awarie_mut = awarie.borrow_mut();
        if id_awarii < awarie_mut.len() {
            awarie_mut.remove(id_awarii);
        } else {
            ic_cdk::api::trap("Index out of bounds");
        }
    });
}

#[update]
fn dodaj_komentarz(id_awarii: usize, komentarz: String) {
    AWARIE.with(|awarie| {
        let mut awarie_mut = awarie.borrow_mut();
        if let Some(awaria) = awarie_mut.get_mut(id_awarii) {
            awaria.komentarze.push(komentarz);
        } else {
            ic_cdk::api::trap("Index out of bounds");
        }
    });
}

#[update]
fn edytuj_awarie(id_awarii: usize, nowa_awaria: String) {
    AWARIE.with(|awarie| {
        let mut binding = awarie.borrow_mut();
        if let Some(existing) = binding.get_mut(id_awarii) {
            existing.nazwa = nowa_awaria;
        } else {
            ic_cdk::api::trap("Index out of bounds");
        }
    });
}
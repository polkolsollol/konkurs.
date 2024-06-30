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
    static AWARIE: RefCell<Vec<String>> = RefCell::default();
    static KOMENTARZE: RefCell<Vec<Vec<String>>> = RefCell::default();
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


#[ic_cdk::update]
fn dodaj_awarie(awaria: String) {
    AWARIE.with(|awarie| {
        awarie.borrow_mut().push(awaria)
    });
    KOMENTARZE.with(|komentarze| {
        komentarze.borrow_mut().push(vec![]);
    });
}

#[ic_cdk::query]
fn odczytaj_awarie() -> Vec<String> {
    AWARIE.with(|awarie| {
        awarie.borrow().clone()
    })
}

#[ic_cdk::update]
fn usun_awarie(id_awarii: usize){
    AWARIE.with(|awarie| {
        awarie.borrow_mut().remove(id_awarii)
    });
    KOMENTARZE.with(|komentarze| {
        komentarze.borrow_mut().remove(id_awarii)
    });
}

#[ic_cdk::update]
fn dodaj_komentarz(id_awarii: usize, komentarz: String){
    KOMENTARZE.with(|komentarze| {
        let mut binding = komentarze.borrow_mut();
        if let Some(komentarze_awarii) = binding.get_mut(id_awarii) {
            komentarze_awarii.push(komentarz);
        } else {
            // Handle the case where id_awarii is out of bounds
            ic_cdk::api::trap("Index out of bounds");
        }
    });
}

#[ic_cdk::query]
fn odczytaj_zgloszenia() -> Vec<String> {
    AWARIE.with(|awarie| {
        awarie.borrow().clone()
    })
}
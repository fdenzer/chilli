// Test the configuration.

extern crate chilli;
extern crate serde_json;

use serde_json::value::ToJson;
use std::env;

use chilli::Pencil;


fn config_test(app: Pencil) {
    let test_key = app.config.get("TEST_KEY").unwrap();
    let secret_key = app.config.get("SECRET_KEY").unwrap();
    assert!(test_key.as_str().unwrap() == "foo");
    assert!(secret_key.as_str().unwrap() == "mysecret");
    assert!(app.config.get("MISSING_KEY") == None);
}


#[test]
fn test_config_basic_set() {
    let mut app = Pencil::new("/test");
    app.config.set("TEST_KEY", "foo".to_json().unwrap());
    app.config.set("SECRET_KEY", "mysecret".to_json().unwrap());
    config_test(app);
}


#[test]
fn test_config_from_object() {
    let mut app = Pencil::new("/test");
    let mut object = serde_json::Map::new();
    object.insert("TEST_KEY".to_string(), "foo".to_json().unwrap());
    object.insert("SECRET_KEY".to_string(), "mysecret".to_json().unwrap());
    app.config.from_object(object);
    config_test(app);
}


#[test]
fn test_config_from_file() {
    let mut app = Pencil::new("/test");
    app.config.from_jsonfile("./tests/test_config.json");
    config_test(app);
}


#[test]
fn test_config_from_envvar() {
    let mut app = Pencil::new("/test");
    env::set_var("PENCIL_TEST_APP_SETTINGS", "./tests/test_config.json");
    app.config.from_envvar("PENCIL_TEST_APP_SETTINGS");
    config_test(app);
    env::remove_var("PENCIL_TEST_APP_SETTINGS");
}

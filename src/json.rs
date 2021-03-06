//! This module implements helpers for the JSON support in Pencil.

use serde::Serialize;
use serde_json;

use wrappers::{Response};
use types::{PencilResult, PenUserError, UserError};


/// Creates a view result with the JSON representation of the given object
/// with an *application/json* mimetype. Example usage:
///
/// ```ignore
/// extern crate rustc_serialize;
///
/// use chilli::{Request, PencilResult, jsonify};
///
/// #[derive(RustcEncodable)]
/// struct User {
///     id: u8,
///     name: String,
/// }
///
/// fn get_user(_: &mut Request) -> PencilResult {
///     let user = User {
///         id: 1,
///         name: String::from("admin"),
///     };
///     return jsonify(&user);
/// }
/// ```
pub fn jsonify<T: Serialize>(object: &T) -> PencilResult {
    match serde_json::to_string(object) {
        Ok(encoded) => {
            let mut response = Response::from(encoded);
            response.set_content_type("application/json");
            Ok(response)
        },
        Err(err) => {
            let error = UserError::new(format!("Json encoder error: {}", err));
            Err(PenUserError(error))
        },
    }
}

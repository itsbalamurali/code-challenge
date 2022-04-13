use poem::{Route, get};

use crate::handlers::{helloworld_handler, versionz_handler};

pub fn app_router() -> Route {
    return Route::new()
        .at("/helloworld", get(helloworld_handler))
        .at("/versionz", get(versionz_handler));
}
use http::{Request, Response};
pub use inventory;
pub use router_macros::route;
pub use router_shared::HttpMethod;

pub type Handler = for<'a> fn(Request<&'a [u8]>) -> Response<Vec<u8>>;

pub struct RegisteredRoute {
    pub handler: Handler,
    pub uri: &'static str,
    pub method: HttpMethod,
}

inventory::collect!(RegisteredRoute);

pub fn route_request(request: Request<&[u8]>) -> Response<Vec<u8>> {
    let uri = request.uri().path();
    let method = request.method();

    for route in inventory::iter::<RegisteredRoute> {
        if (route.uri == uri) && (route.method == *method) {
            return (route.handler)(request);
        }
    }

    // If we don't have a route setup for this URI + method combo, return a 404
    Response::builder().status(404).body(Vec::new()).unwrap()
}


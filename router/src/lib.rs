pub use inventory;
pub use router_macros::route;

pub struct RegisteredRoute {
    pub name: &'static str,
    pub uri: &'static str,
}

inventory::collect!(RegisteredRoute);

/// The "global variable": every registered route function, collected at link time.
pub fn registered() -> impl Iterator<Item = &'static RegisteredRoute> {
    inventory::iter::<RegisteredRoute>.into_iter()
}

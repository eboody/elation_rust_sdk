pub trait Resource {
    /// The type of the resource's identifier (e.g., i64).
    type Id: ToString;

    /// Returns the API endpoint for the resource.
    fn endpoint() -> &'static str;
}

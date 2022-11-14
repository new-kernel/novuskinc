/// Used for interacting with [``.dif``](todo) files
extern "C" {
    /// Converts ``field`` to a
    /// [``DifFieldNames``](https://docs.rs/dif/latest/dif/fields/enum.DifFieldNames.html) and
    /// returns it's value from the ``.dif`` file.
    pub fn get_dif_value(field: &'static str) -> &'static str;
}
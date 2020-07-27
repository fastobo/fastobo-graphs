pub mod serde {
    use serde::Deserialize;
    use serde::Deserializer;

    /// Deserialize a possibly missing vector into an empty one.
    pub fn optional_vector<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        match Option::deserialize(deserializer) {
            Ok(Some(v)) => Ok(v),
            Ok(None) => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    /// Deserialize a vector possibly containing `null`.
    pub fn nullable_vector<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        Vec::<Option<T>>::deserialize(deserializer).map(|v| v.into_iter().flatten().collect())
    }
}

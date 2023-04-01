use std::ffi::OsString;
#[cfg(test)]
use std::collections::HashMap;
#[cfg(not(test))]
use std::env;

#[derive(Default)]
pub struct Env<'a> {
    /// A HashMap of environment variable mocks
    #[cfg(test)]
    pub env: HashMap<&'a str, String>,

    #[cfg(not(test))]
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Env<'a> {
    // Retrieves a environment variable from the os or from a table if in testing mode
    #[cfg(test)]
    pub fn get_env<K: AsRef<str>>(&self, key: K) -> Option<String> {
        self.env
            .get(key.as_ref())
            .map(std::string::ToString::to_string)
    }

    #[cfg(not(test))]
    #[inline]
    pub fn get_env<K: AsRef<str>>(&self, key: K) -> Option<String> {
        env::var(key.as_ref()).ok()
    }

    // Retrieves a environment variable from the os or from a table if in testing mode (os version)
    #[cfg(test)]
    pub fn get_env_os<K: AsRef<str>>(&self, key: K) -> Option<OsString> {
        self.env.get(key.as_ref()).map(OsString::from)
    }

    #[cfg(not(test))]
    #[inline]
    pub fn get_env_os<K: AsRef<str>>(&self, key: K) -> Option<OsString> {
        env::var_os(key.as_ref())
    }

    #[cfg(test)]
    pub fn insert(&mut self, k: &'a str, v: String) -> Option<String> {
        self.env.insert(k, v)
    }
}
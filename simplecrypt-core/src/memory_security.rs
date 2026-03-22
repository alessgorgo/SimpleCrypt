use zeroize::Zeroize;

/// Memory security utilities for handling sensitive data
pub struct MemorySecurity;

impl MemorySecurity {
    /// Securely wipe a byte slice from memory
    pub fn secure_wipe(data: &mut [u8]) {
        data.zeroize();
    }

    /// Securely wipe a string from memory
    pub fn secure_wipe_string(s: &mut String) {
        s.zeroize();
    }

    /// Securely wipe a vector from memory
    pub fn secure_wipe_vec<T: Zeroize>(v: &mut Vec<T>) {
        v.zeroize();
    }

    /// Clear process environment variables that might contain sensitive data
    pub fn clear_sensitive_env_vars() {
        let sensitive_vars = [
            "PASSWORD",
            "SECRET",
            "KEY",
            "TOKEN",
            "CREDENTIALS",
            "API_KEY",
            "AUTH_TOKEN",
        ];

        for var in &sensitive_vars {
            std::env::remove_var(*var);
        }
    }

    /// Set secure file permissions (600 - read/write for owner only)
    pub fn set_secure_file_permissions(path: &std::path::Path) -> std::io::Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let mut perms = std::fs::metadata(path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(path, perms)?;
        }

        #[cfg(windows)]
        {
            let perms = std::fs::metadata(path)?.permissions();
            std::fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    /// Securely compare two byte slices in constant time to prevent timing attacks
    pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }

    /// Securely compare two strings in constant time
    pub fn constant_time_compare_strings(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }

        Self::constant_time_compare(a.as_bytes(), b.as_bytes())
    }
}

/// A guard that automatically wipes memory when dropped
pub struct SecureMemoryGuard<T: Zeroize> {
    data: Option<T>,
}

impl<T: Zeroize> SecureMemoryGuard<T> {
    pub fn new(data: T) -> Self {
        Self { data: Some(data) }
    }

    pub fn as_ref(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.data.as_mut()
    }

    pub fn into_inner(mut self) -> Option<T> {
        self.data.take()
    }
}

impl<T: Zeroize> Drop for SecureMemoryGuard<T> {
    fn drop(&mut self) {
        if let Some(ref mut data) = self.data {
            data.zeroize();
        }
    }
}

/// A secure string that automatically wipes memory when dropped
pub struct SecureString {
    inner: String,
}

impl SecureString {
    pub fn new() -> Self {
        Self { inner: String::new() }
    }

    pub fn from(s: &str) -> Self {
        Self { inner: s.to_string() }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        &mut self.inner
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl Default for SecureString {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for SecureString {
    fn from(s: String) -> Self {
        Self { inner: s }
    }
}

impl Drop for SecureString {
    fn drop(&mut self) {
        MemorySecurity::secure_wipe_string(&mut self.inner);
    }
}

impl std::fmt::Debug for SecureString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SecureString(len={})", self.inner.len())
    }
}

impl std::ops::Deref for SecureString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::convert::AsRef<str> for SecureString {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_wipe() {
        let mut data = vec![1u8, 2u8, 3u8, 4u8, 5u8];
        MemorySecurity::secure_wipe(&mut data);
        assert_eq!(data, vec![0u8; 5]);
    }

    #[test]
    fn test_secure_memory_guard() {
        let mut guard = SecureMemoryGuard::new(vec![1u8, 2u8, 3u8]);
        assert_eq!(guard.as_ref(), Some(&vec![1u8, 2u8, 3u8]));

        guard.as_mut().map(|data| data.push(4u8));
        assert_eq!(guard.as_ref(), Some(&vec![1u8, 2u8, 3u8, 4u8]));

        let data = guard.into_inner();
        assert_eq!(data, Some(vec![1u8, 2u8, 3u8, 4u8]));
    }

    #[test]
    fn test_secure_string() {
        let secure_str = SecureString::from("test");
        assert_eq!(secure_str.as_str(), "test");
        assert_eq!(secure_str.len(), 4);

        let mut secure_str = SecureString::from("test");
        secure_str.clear();
        assert!(secure_str.is_empty());
    }

    #[test]
    fn test_constant_time_compare() {
        let a = vec![1u8, 2u8, 3u8];
        let b = vec![1u8, 2u8, 3u8];
        let c = vec![1u8, 2u8, 4u8];

        assert!(MemorySecurity::constant_time_compare(&a, &b));
        assert!(!MemorySecurity::constant_time_compare(&a, &c));

        let d = vec![1u8, 2u8];
        assert!(!MemorySecurity::constant_time_compare(&a, &d));
    }
}

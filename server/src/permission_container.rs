use crate::utils::auth::AccessKey;

#[derive(Clone)]
pub struct PermissionContainer(pub AccessKey);

impl PermissionContainer {
    fn has_permission(&self, domain: &str, action: &str, resource: &str) -> bool {
        if !self.0.is_active {
            return false;
        }

        let perms = match &self.0.metadata.permissions {
            Some(p) if !p.is_empty() => p,
            _ => return true,
        };

        perms
            .iter()
            .any(|p| Self::perm_matches(p, domain, action, resource))
    }

    fn perm_matches(pattern: &str, domain: &str, action: &str, resource: &str) -> bool {
        let mut parts = pattern.split(':');
        let (d, a, r) = match (parts.next(), parts.next(), parts.next()) {
            (Some(d), Some(a), Some(r)) => (d.trim(), a.trim(), r.trim()),
            _ => return false,
        };

        if parts.next().is_some() {
            return false; // extra segments, malformed
        }

        d == domain && a == action && (r == "*" || r == resource)
    }

    pub fn bucket_can_read(&self, bucket: &str) -> bool {
        self.has_permission("bucket", "read", bucket)
    }

    pub fn bucket_can_write(&self, bucket: &str) -> bool {
        self.has_permission("bucket", "write", bucket)
    }

    pub fn bucket_can_create(&self, bucket: &str) -> bool {
        self.has_permission("bucket", "create", bucket)
    }

    pub fn bucket_can_delete(&self, bucket: &str) -> bool {
        self.has_permission("bucket", "delete", bucket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::auth::KeyMetadata;

    fn make_key(perms: &[&str], active: bool) -> PermissionContainer {
        PermissionContainer(AccessKey {
            id: 1,
            owner_id: 1,
            access_key: "abc123".into(),
            created_at: "2025-01-01T00:00:00Z".into(),
            is_active: active,
            metadata: KeyMetadata {
                permissions: Some(perms.iter().map(|s| s.to_string()).collect()),
            },
        })
    }

    #[test]
    fn read_any_bucket_with_wildcard() {
        let container = make_key(&["bucket:read:*"], true);
        assert!(container.bucket_can_read("photos"));
        assert!(container.bucket_can_read("videos"));
        assert!(!container.bucket_can_write("photos"));
    }

    #[test]
    fn read_specific_bucket() {
        let container = make_key(&["bucket:read:photos"], true);
        assert!(container.bucket_can_read("photos"));
        assert!(!container.bucket_can_read("videos"));
    }

    #[test]
    fn write_permission_does_not_grant_read() {
        let container = make_key(&["bucket:write:photos"], true);
        assert!(container.bucket_can_write("photos"));
        assert!(!container.bucket_can_read("photos"));
    }

    #[test]
    fn multiple_permissions() {
        let container = make_key(&["bucket:read:photos", "bucket:write:*"], true);
        assert!(container.bucket_can_read("photos"));
        assert!(container.bucket_can_write("anybucket"));
        assert!(!container.bucket_can_read("videos")); // no read access there
    }

    #[test]
    fn inactive_key_blocks_access() {
        let container = make_key(&["bucket:read:*"], false);
        assert!(!container.bucket_can_read("photos"));
        assert!(!container.bucket_can_write("videos"));
    }

    #[test]
    fn malformed_permission_is_ignored() {
        let container = make_key(&["bucket:read", "bucket:read:*"], true);
        assert!(container.bucket_can_read("photos")); // second one still valid
    }

    #[test]
    fn empty_permissions_block_access() {
        let container = make_key(&[], true);
        assert!(!container.bucket_can_read("photos"));
    }

    #[test]
    fn none_permissions_block_access() {
        let mut key = make_key(&[], true);
        key.0.metadata.permissions = None;
        assert!(!key.bucket_can_read("photos"));
    }
}

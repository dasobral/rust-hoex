//! Recursive security policy rules using `Box`.

/// A composable access-control rule evaluated against a subject string.
///
/// Recursive variants use `Box` so the enum has a finite size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    /// Permit when the subject matches `action`.
    Allow(String),
    /// Permit when the subject does **not** match `action`.
    Deny(String),
    /// Both child rules must permit.
    AndThen(Box<Self>, Box<Self>),
    /// Either child rule may permit.
    Or(Box<Self>, Box<Self>),
}

impl Rule {
    /// Evaluate whether `subject` is permitted by this rule tree.
    #[must_use]
    pub fn allows(&self, subject: &str) -> bool {
        match self {
            Self::Allow(action) => subject == action,
            Self::Deny(action) => subject != action,
            Self::AndThen(left, right) => left.allows(subject) && right.allows(subject),
            Self::Or(left, right) => left.allows(subject) || right.allows(subject),
        }
    }

    /// Count nodes in the tree (walks through `Box` indirection).
    #[must_use]
    pub fn node_count(&self) -> usize {
        match self {
            Self::Allow(_) | Self::Deny(_) => 1,
            Self::AndThen(left, right) | Self::Or(left, right) => {
                1 + left.node_count() + right.node_count()
            }
        }
    }
}

/// Sample policy: allow read or list, but never delete.
#[must_use]
pub fn sample_policy() -> Rule {
    Rule::AndThen(
        Box::new(Rule::Or(
            Box::new(Rule::Allow("read".into())),
            Box::new(Rule::Allow("list".into())),
        )),
        Box::new(Rule::Deny("delete".into())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allow_matches_subject() {
        assert!(Rule::Allow("ping".into()).allows("ping"));
        assert!(!Rule::Allow("ping".into()).allows("pong"));
    }

    #[test]
    fn deny_inverts_match() {
        assert!(Rule::Deny("delete".into()).allows("read"));
        assert!(!Rule::Deny("delete".into()).allows("delete"));
    }

    #[test]
    fn and_then_requires_both() {
        let rule = Rule::AndThen(
            Box::new(Rule::Allow("read".into())),
            Box::new(Rule::Deny("delete".into())),
        );
        assert!(rule.allows("read"));
        assert!(!rule.allows("delete"));
    }

    #[test]
    fn or_accepts_either_branch() {
        let rule = Rule::Or(
            Box::new(Rule::Allow("read".into())),
            Box::new(Rule::Allow("write".into())),
        );
        assert!(rule.allows("read"));
        assert!(rule.allows("write"));
        assert!(!rule.allows("delete"));
    }

    #[test]
    fn sample_policy_blocks_delete() {
        let policy = sample_policy();
        assert!(policy.allows("read"));
        assert!(policy.allows("list"));
        assert!(!policy.allows("delete"));
        assert!(policy.node_count() >= 4);
    }
}

//! testing fundamental types
use unsynn::*;

#[test]
fn test_cached() {
    let mut tokens = "test".to_token_iter();
    let mut cached = Cached::<Ident>::parse(&mut tokens).unwrap();

    // Test string representations
    assert_eq!(cached.as_str(), "test");
    assert_tokens_eq!(cached, "test");

    // Test modification
    cached.set(Ident::new("modified", Span::call_site()));
    assert_eq!(cached.as_str(), "modified");

    // Test Deref
    assert_eq!(cached.to_string(), "modified");

    // Test as_str
    assert_eq!(cached.as_str(), "modified");

    // Test conversion to TokenTree
    let tt: TokenTree = cached.into();
    assert_tokens_eq!(tt, "modified");
}

#[test]
fn test_cached_comparisons() {
    let mut tokens = "test".to_token_iter();
    let cached1 = Cached::<Ident>::parse(&mut tokens).unwrap();
    let mut tokens = "test".to_token_iter();
    let cached2 = Cached::<Ident>::parse(&mut tokens).unwrap();
    let mut tokens = "other".to_token_iter();
    let cached3 = Cached::<Ident>::parse(&mut tokens).unwrap();

    // Test PartialEq
    assert_eq!(cached1, cached2);
    assert_ne!(cached1, cached3);

    // Test PartialEq with str
    assert_eq!(cached1, "test");
    assert_ne!(cached1, "other");
}

#[test]
fn test_nothing() {
    let mut tokens = "test".to_token_iter();
    let nothing = Nothing::parse(&mut tokens).unwrap();

    // Verify Nothing doesn't consume tokens
    assert_tokens_eq!(tokens.next().unwrap(), "test");

    // Test ToTokens
    let mut output = TokenStream::new();
    nothing.to_tokens(&mut output);
    assert!(output.is_empty());
}

#[test]
#[should_panic = "`Invalid` can not be converted to tokens"]
fn test_invalid() {
    let mut tokens = "test".to_token_iter();
    assert!(Invalid::parse(&mut tokens).is_err());

    // Test ToTokens
    let mut output = TokenStream::new();
    Invalid.to_tokens(&mut output);
    assert!(output.is_empty());
}

#[test]
fn test_except() {
    let mut tokens = "test".to_token_iter();

    // Should succeed when token doesn't match specified type
    assert!(Except::<Punct>::parse(&mut tokens).is_ok());

    // Should fail when token matches specified type
    assert!(Except::<Ident>::parse(&mut tokens).is_err());

    // Verify tokens weren't consumed
    assert_tokens_eq!(tokens.next().unwrap(), "test");
}

#[test]
fn test_expect() {
    let mut tokens = "test".to_token_iter();

    // Should succeed when token matches specified type
    assert!(Expect::<Ident>::parse(&mut tokens).is_ok());

    // Should fail when token doesn't match specified type
    assert!(Expect::<Punct>::parse(&mut tokens).is_err());

    // Verify tokens weren't consumed
    assert_tokens_eq!(tokens.next().unwrap(), "test");
}

#[test]
fn test_end_of_stream() {
    let mut tokens = "test".to_token_iter();

    // Should fail when tokens remain
    assert!(EndOfStream::parse(&mut tokens).is_err());

    // Consume token
    tokens.next();

    // Should succeed when no tokens remain
    assert!(EndOfStream::parse(&mut tokens).is_ok());
}

#[test]
fn test_hidden_state() {
    #[derive(Default)]
    struct TestState {
        value: i32,
    }

    let mut tokens = "test".to_token_iter();
    let mut state = HiddenState::<TestState>::parse(&mut tokens).unwrap();

    // Test default initialization
    assert_eq!(state.value, 0);

    // Test mutation through DerefMut
    state.value = 42;
    assert_eq!(state.value, 42);

    // Test ToTokens produces empty stream
    let mut output = TokenStream::new();
    state.to_tokens(&mut output);
    assert!(output.is_empty());

    // Verify tokens weren't consumed
    assert_tokens_eq!(tokens.next().unwrap(), "test");
}

#[test]
fn test_non_empty_token_stream() {
    let mut tokens = "test".to_token_iter();
    let nes = NonEmptyTokenStream::parse(&mut tokens).unwrap();

    // Test ToTokens implementation
    let mut output = TokenStream::new();
    nes.to_tokens(&mut output);
    assert!(!output.is_empty());
    assert_eq!(output.to_string(), "test");
}

#[test]
fn test_cached_string_methods() {
    let mut tokens = "test".to_token_iter();
    let cached = Cached::<Ident>::parse(&mut tokens).unwrap();

    // Test string representation methods
    assert_eq!(cached.as_str(), "test");
    assert_ne!(cached.as_str(), ""); // Catch empty string mutation
    assert_ne!(cached.as_str(), "xyzzy"); // Catch "xyzzy" mutation

    // Test AsRef<str> implementation
    let s: &str = cached.as_ref();
    assert_eq!(s, "test");
    assert_ne!(s, ""); // Catch empty string mutation
    assert_ne!(s, "xyzzy"); // Catch "xyzzy" mutation

    // Test into_string
    let s = cached.into_string();
    assert_eq!(s, "test");
    assert_ne!(s, ""); // Catch String::new() mutation
}

#[test]
fn test_cached_hash() {
    use std::collections::HashSet;

    let mut tokens = "test".to_token_iter();
    let cached1 = Cached::<Ident>::parse(&mut tokens).unwrap();
    let mut tokens = "test".to_token_iter();
    let cached2 = Cached::<Ident>::parse(&mut tokens).unwrap();
    let mut tokens = "other".to_token_iter();
    let cached3 = Cached::<Ident>::parse(&mut tokens).unwrap();

    // Test Hash implementation by using a HashSet
    let mut set = HashSet::new();
    set.insert(cached1);

    // Same content should hash to same value
    assert!(!set.insert(cached2));

    // Different content should hash to different value
    assert!(set.insert(cached3));
}

#[test]
fn test_cached_hash_behavior() {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    let mut tokens = "test".to_token_iter();
    let cached1 = Cached::<Ident>::parse(&mut tokens).unwrap();

    let mut tokens = "test".to_token_iter();
    let cached2 = Cached::<Ident>::parse(&mut tokens).unwrap();

    // Calculate hashes explicitly
    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();
    cached1.hash(&mut hasher1);
    cached2.hash(&mut hasher2);

    // Same content should hash to same value
    let hash1 = hasher1.finish();
    let hash2 = hasher2.finish();
    assert_eq!(hash1, hash2);

    // Verify hash affects set behavior
    let mut set = HashSet::new();
    assert!(set.insert(cached1));
    assert!(!set.insert(cached2)); // Would fail if hash() was replaced with ()
}

#[test]
fn test_hidden_state_parser() {
    #[derive(Default, PartialEq, Debug)]
    struct TestState {
        value: i32,
    }

    let mut tokens = "test".to_token_iter();
    let state1 = HiddenState::<TestState>::parse(&mut tokens).unwrap();

    // Create a second state to compare against default
    let state2 = HiddenState(TestState::default());

    // They should be equal but not the same instance
    assert_eq!(state1.value, state2.value);

    // Original token should still be available (parser shouldn't consume it)
    assert_tokens_eq!(tokens.next().unwrap(), "test");
}

#[test]
fn test_cached_string_content() {
    let mut tokens = "test_value".to_token_iter();
    let cached = Cached::<Ident>::parse(&mut tokens).unwrap();

    // Test exact string content - should fail if replaced with "xyzzy" or ""
    let s = cached.as_str();
    assert_eq!(s.len(), "test_value".len()); // Would fail for both "" and "xyzzy"
    assert_eq!(s.chars().count(), "test_value".chars().count()); // Double-check length
    assert!(s.contains("test")); // Would fail for both mutations
}

#[test]
fn test_cached_hash_identity() {
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    let mut tokens = "same".to_token_iter();
    let cached1 = Cached::<Ident>::parse(&mut tokens).unwrap();
    let mut tokens = "same".to_token_iter();
    let cached2 = Cached::<Ident>::parse(&mut tokens).unwrap();
    let mut tokens = "different".to_token_iter();
    let cached3 = Cached::<Ident>::parse(&mut tokens).unwrap();

    // Get hash values
    let hash = |value: &Cached<Ident>| {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    };

    // Same content should have same hash
    assert_eq!(hash(&cached1), hash(&cached2));
    // Different content should have different hash
    assert_ne!(hash(&cached1), hash(&cached3));

    // Verify hash affects set behavior
    let mut set = HashSet::new();
    assert!(set.insert(cached1));
    assert!(!set.insert(cached2));
    assert!(set.insert(cached3));
}

#[test]
fn test_debug_impls() {
    let mut tokens = "test".to_token_iter();
    let cached = Cached::<Ident>::parse(&mut tokens).unwrap();
    let debug_str = format!("{cached:?}");
    assert!(debug_str.contains("test"));

    let nothing = Nothing;
    let debug_str = format!("{nothing:?}");
    assert!(debug_str.contains("Nothing"));

    let invalid = Invalid;
    let debug_str = format!("{invalid:?}");
    assert!(debug_str.contains("Invalid"));
}

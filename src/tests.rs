use super::*;

#[test]
fn eq() {
    assert_eq!(RawStr::from_str("Hello!"), "Hello!");
    assert_ne!(RawStr::from_str("Hello!"), "hello!");
    assert_ne!(RawStr::from_str("Hello!"), "");
    assert_ne!(RawStr::from_str("Hello!"), "Hello! there!");
}

#[test]
fn case_eq() {
    assert!(RawStr::from_str("Hello!").eq_ignore_ascii_case("Hello!".into()));
    assert!(RawStr::from_str("Hello!").eq_ignore_ascii_case("hello!".into()));
    assert!(!RawStr::from_str("Hello!").eq_ignore_ascii_case("Hello! there!".into()));
}

#[cfg(feature = "std")]
#[test]
fn display_fmt() {
    assert_eq!(
        format!("foo {} bar", RawStr::from_str("Hello!")),
        "foo Hello! bar"
    );
}

#[cfg(feature = "std")]
#[test]
fn debug_fmt() {
    assert_eq!(
        format!("foo {:?} bar", RawStr::from_str("Hello\t!")),
        "foo \"Hello\\t!\" bar"
    );
}

#[test]
fn ends_with() {
    assert!(RawStr::from_bytes(b"Hello!").ends_with(RawStr::from_bytes(b"lo!")));
    assert!(!RawStr::from_bytes(b"Hello!").ends_with(RawStr::from_bytes(b"yo!")));
}

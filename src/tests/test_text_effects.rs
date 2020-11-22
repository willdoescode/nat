use crate::*;

#[test]
fn test_bold() {
  assert_eq!(
    text_effects::bold("hello"),
    String::from("\x1B[1mhello\x1B[0m")
  )
}

#[test]
fn test_underline() {
  assert_eq!(
    text_effects::underline("hello"),
    String::from("\x1B[4mhello\x1B[0m")
  )
}

#[test]
fn test_dimmed() {
  assert_eq!(
    text_effects::dimmed("hello"),
    String::from("\x1B[2mhello\x1B[0m")
  )
}

#[test]
fn test_italic() {
  assert_eq!(
    text_effects::italic("hello"),
    String::from("\x1B[3mhello\x1B[0m")
  )
}

#[test]
fn test_blink() {
  assert_eq!(
    text_effects::blink("hello"),
    String::from("\x1B[5mhello\x1B[0m")
  )
}

#[test]
fn test_reverse() {
  assert_eq!(
    text_effects::reverse("hello"),
    String::from("\x1B[7mhello\x1B[0m")
  )
}

#[test]
fn test_hidden() {
  assert_eq!(
    text_effects::hidden("hello"),
    String::from("\x1B[8mhello\x1B[0m")
  )
}

#[test]
fn test_stricken() {
  assert_eq!(
    text_effects::stricken("hello"),
    String::from("\x1B[9mhello\x1B[0m")
  )
}

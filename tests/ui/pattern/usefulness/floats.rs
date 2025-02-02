#![allow(illegal_floating_point_literal_pattern)]
#![deny(unreachable_patterns)]

fn main() {
    match 0.0 {
      0.0..=1.0 => {}
      _ => {} // ok
    }

    match 0.0 { //~ ERROR non-exhaustive patterns
      0.0..=1.0 => {}
    }

    match 1.0f64 {
      0.01f64 ..= 6.5f64 => {}
      0.02f64 => {} //~ ERROR unreachable pattern
      _ => {}
    };
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal Patterns
//
// ferrocene-annotations: fls_azzf1llv3wf
// Literal Pattern Matching
//
// ferrocene-annotations: fls_6tl1fx99yn6c
// Range Patterns
//
// ferrocene-annotations: fls_fyskeih6twyb
// Range Pattern Matching

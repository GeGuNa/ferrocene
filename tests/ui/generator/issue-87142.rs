// compile-flags: -Cdebuginfo=2
// build-pass

// Regression test for #87142
// This test needs the above flags and the "lib" crate type.

#![feature(impl_trait_in_assoc_type, generator_trait, generators)]
#![crate_type = "lib"]

use std::ops::Generator;

pub trait GeneratorProviderAlt: Sized {
    type Gen: Generator<(), Return = (), Yield = ()>;

    fn start(ctx: Context<Self>) -> Self::Gen;
}

pub struct Context<G: 'static + GeneratorProviderAlt> {
    pub link: Box<G::Gen>,
}

impl GeneratorProviderAlt for () {
    type Gen = impl Generator<(), Return = (), Yield = ()>;
    fn start(ctx: Context<Self>) -> Self::Gen {
        move || {
            match ctx {
                _ => (),
            }
            yield ();
        }
    }
}

// ferrocene-annotations: um_rustc_C_debuginfo

pub mod defer {

    /// go-like defer provider
    pub struct Defer<F: FnMut()> {
        /// proc to do
        pub defer_proc: Option<F>,
    }

    impl<F: FnMut()> Drop for Defer<F> {
        /// do defer-proc when drop object
        fn drop(&mut self) {
            self.defer_proc.take().unwrap()()
        }
    }

    impl<F: FnMut()> Defer<F> {
        /// register defer proc
        pub fn register(p: F) -> Defer<F> {
            Defer {
                defer_proc: Some(p),
            }
        }
    }

    /// # Macro
    ///
    /// Usage:
    ///
    /// ```
    /// #[macro_use(defer)]
    /// extern crate defer;
    ///
    /// use defer::Defer;
    ///
    /// fn main() {
    ///     defer!({
    ///         println!("this will appear 4th!");
    ///     });
    ///     defer!({
    ///         println!("this will appear 2nd!");
    ///         println!("this will appear 3rd!");
    ///     });
    ///     println!("this will appear 1st!");
    /// }
    /// ```
    #[macro_export]
    macro_rules! defer {
        ($e:expr) => (
            let _scope_call = Defer::register(
                || -> () { $e; }
            );
        )
    }
}

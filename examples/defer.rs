struct ScopeCall<F: FnOnce()> {
    c: Option<F>
}
impl<F: FnOnce()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        self.c.take().unwrap()()
    }
}

macro_rules! expr { ($e: expr) => { $e } } // tt hack
macro_rules! defer {
    ($($data: tt)*) => (
        let _scope_call = ScopeCall {
            c: Some(|| -> () { expr!({ $($data)* }) })
        };
    )
}

fn main() {
    let x = 42u8;
    defer!(println!("defer 1"));
    defer! {
        println!("defer 2");
        println!("inside defer {}", x)
    }
    println!("normal execution {}", x);
}

#[macro_export]
macro_rules! myvec {
    // trailing comma is allowed
    ($($e: expr),* $(,)?) => {{
        const LEN: usize = $crate::myvec![@COUNT; $($e),*];
        #[allow(unused_mut)]
        let mut res = Vec::with_capacity(LEN);
        $(res.push($e);)*
        res
    }};
    ($element:expr; $count:expr) => {{
        let count = $count;
        let mut res = Vec::with_capacity(count);
        res.extend(std::iter::repeat($element).take(count));
        // or
        // let mut res = Vec::new();
        // res.resize($count, $element);
        res
    }};
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::myvec![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}

trait MaxValue {
    fn max_value() -> Self;
}

#[macro_export]
macro_rules! max_impl {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo expand --lib --tests
    #[test]
    fn empty_vec() {
        let x: Vec<u32> = myvec![];
        assert!(x.is_empty());
    }

    #[test]
    fn single() {
        let x: Vec<u32> = myvec![42];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn double() {
        let x: Vec<u32> = myvec![42, 43,];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 43);
    }

    #[test]
    fn clone2_nonliteral() {
        let mut y = Some(42);
        let x: Vec<u32> = myvec![y.take().unwrap(); 2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }
}

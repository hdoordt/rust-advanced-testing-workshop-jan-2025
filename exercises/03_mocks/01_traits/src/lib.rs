//! Refactor the `square` function to ask for a type that implements the `Logger` trait rather than the concrete
//! `PrintlnLogger` type.\
//! Then pass a `TestLogger` to `square` in the test. `TestLogger` should implement `Logger` and do nothing
//! when `log` is called.

pub fn square<L>(x: i32, logger: L) -> i32 where L: Logger {
    let y = x * x;
    logger.log(&format!("{}^2 == {}", x, y));
    y
}

pub struct MockLogger;

pub trait Logger {
    fn log(&self, msg: &str);
}

impl Logger for MockLogger {
    fn log(&self, _msg: &str) {
        //...
    }
}

pub struct PrintlnLogger;

impl PrintlnLogger {
    pub fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::square;
    use googletest::assert_that;
    use googletest::matchers::eq;
    use super::MockLogger;

    #[test]
    fn square_works() {
        assert_eq!(square(2, MockLogger), 4);
    }
}

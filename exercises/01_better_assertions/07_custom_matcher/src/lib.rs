//! Write a custom `is_redirect` matcher that checks if a `StatusCode` is a redirect.
use googletest::matcher::{Matcher, MatcherBase};
use http::StatusCode;

pub fn is_redirect() -> impl Matcher<StatusCode> {
    RedirectMatcher
}

#[derive(MatcherBase)]
struct RedirectMatcher;

impl Matcher<StatusCode> for RedirectMatcher {

    fn describe(&self, matcher_result: googletest::matcher::MatcherResult) -> googletest::description::Description {
        match matcher_result {
            googletest::matcher::MatcherResult::Match => googletest::description::Description::new().text("is a redirection status code"),
            googletest::matcher::MatcherResult::NoMatch => googletest::description::Description::new().text("isn't a redirection status code")
        }
    }

    fn matches(&self, actual: StatusCode) -> googletest::matcher::MatcherResult {
        match actual.is_redirection() {
            true => googletest::matcher::MatcherResult::Match,
            false => googletest::matcher::MatcherResult::NoMatch
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::is_redirect;
    use googletest::assert_that;
    use http::StatusCode;

    #[test]
    fn success() {
        assert_that!(StatusCode::MOVED_PERMANENTLY, is_redirect());
    }

    #[test]
    fn failure() {
        assert_that!(StatusCode::OK, is_redirect());
    }
}

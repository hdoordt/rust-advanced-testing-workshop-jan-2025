//! Replace the `assert!` calls with the corresponding `googletest` matchers.
#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::{container_eq, contains, each, eq, gt, has_entry, subset_of, anything};
    // The standard library doesn't have a macro for creating `HashMap`s and `BTreeSet`s,
    // so we use the `maplit` to fill the gap.
    use maplit::{btreeset, hashmap};

    #[googletest::gtest]
    fn failed_is_eq() {
        let x = vec![1, 2, 3];
        let y = vec![4, 2, 6];
        // The `eq` matcher works for collections as well, but you can do better than that.
        // Use the container-specific equality matcher instead and check the output!
        assert_that!(x, container_eq(y));
    }

    #[googletest::gtest]
    fn failed_contains() {
        let x = btreeset![1, 2, 3];
        let y = 7;
        assert_that!(x, contains(eq(&y)));
    }

    #[googletest::gtest]
    fn failed_subset_of() {
        let x = btreeset![&1, &2, &3];
        let y = btreeset![3, 4];

        //  assert!(y.is_subset(&x)); <-- std assertion, WRONG!

        assert_that!(y, subset_of(x));
    
        //  assert_that!(y, /* matcher from https://docs.rs/googletest/0.13.0/googletest/index.html#available-matchers */);
    }

    #[googletest::gtest]
    fn failed_each() {
        let x = btreeset![1, 2, 3, 4];
        //  assert!(x.iter().all(|x| *x > 2));
        assert_that!(x, each(gt(&2)));

        // assert_that!(x, /* matcher from https://docs.rs/googletest/0.13.0/googletest/index.html#available-matchers */);
    }

    #[googletest::gtest]
    fn successful_has_entry() {
        let x = hashmap!["a" => 1, "b" => 2];
        let y = "b";
        // Write an assertion equivalent to: `assert!(x.get(y).is_some())`
        assert_that!(x, has_entry(y, anything()));
    }
}

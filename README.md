# webassembly-test

This crate implements a `cargo test` support for `wasm32-unknown-unknown`
target:

```console
$ cat src/lib.rs
#[cfg(test)]
mod tests {
    use webassembly_test::webassembly_test;

    #[webassembly_test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[webassembly_test]
    fn it_does_not_work() {
        assert_eq!(2 + 2, 5);
    }

    #[webassembly_test]
    #[ignore]
    fn it_is_ignored() {
        assert_eq!(2 + 2, 5);
    }
}

$ cargo test --target wasm32-unknown-unknown
     Running `webassembly-test-runner target/wasm32-unknown-unknown/debug/deps/hello_world.wasm`

running 3 tests
test hello_world::tests::it_works ... ok
test hello_world::tests::it_does_not_work ... FAILED
test hello_world::tests::it_is_ignored ... ignored

test result: FAILED. 1 passed; 1 failed; 1 ignored;
```

`webassembly-test` is independent from any particular wasm runtime or
environment. In fact, it is more of a pattern rather than a library, and can be
easily adopted to a particular use-case.

MSRV: 1.54.0 (**beta** at the time of writing).

## Writing Tests

When writing tests, use `#[webassembly_test]` rather than the usual `#[test]`
macro. Add `.cargo/config` which sets a runner for `wasm32-unknown-unknown`:

```toml
[target.wasm32-unknown-unknown]
runner = "webassembly-test-runner"
```

Now, just `cargo test --target wasm32-unknown-unknown` will run the tests.

`webassembly-test-runner` is an example of a runner. You can install it with
`cargo install webassembly-test-runner`. It uses wasmtime to run the tests in an
empty environment. If the tested library needs environment-specific imports you
need to write the runner yourself.

[Example.](https://github.com/matklad/webassembly-test/tree/master/examples/hello-world/)

## Implementing Your Own Runner

The `webassembly_test` macro is simple. For each test, it emits a corresponding
wasm export with a name in special format. For the example, the names would be:

```text
$webassembly-test$hello_world::tests::it_works
$webassembly-test$hello_world::tests::it_does_not_work
$webassembly-test$ignore$hello_world::tests::it_is_ignored
```
That is, `$webassembly-test$` prefix, followed by `ignore$`, followed by the
qualified name of the test function.

The test runner than loads a wasm modules, and calls all export which match the
given format given format.

[Example.](https://github.com/matklad/webassembly-test/tree/master/webassembly-test-runner/)

## Implementation notes

`webassembly_test` uses `concat!(module_path!(), #name)` to set `#[export_name]`
for test functions. The `#[ignore]` is encoded into a function name. Ideally,
we'd use a custom wasm section to store the ignored attributes, but it seems
impossible to correctly associate a custom section entry with full function name
(proc macro doesn't have access to the module path of the function). Sample
runner runs all the tests sequentially in a singe instance, but it should be
possible to run tests in parallel in several instances.

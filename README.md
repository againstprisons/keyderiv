# earmms_keyderiv

The key derivation microservice for [earmms][].

[![Build Status][]][Travis CI]

## Using

* Run `cargo build --release`
* Set the environment variables `INDEX_KEY` and `ENCRYPT_KEY` to hex-encoded
  32-byte values (so the strings will be 64 characters long)
* Set the environment variable `PORT` to the port number to listen on
  * Alternatively, for development, don't set `PORT` and instead run the binary
    under [catflap][].
* Run `./target/release/earmms_keyderiv`
* Set the `KEYDERIV_URL` in your earmms config to point to the port set above

## Running the tests

```shell
$ cargo install catflap # if you don't have catflap already
$ catflap -- ./tests/run-tests.sh
```

## License

MIT, see [LICENSE][].

[LICENSE]: ./LICENSE
[earmms]: https://github.com/peopleagainstprisons/earmms
[Build Status]: https://travis-ci.org/peopleagainstprisons/earmms_keyderiv.svg?branch=master
[Travis CI]: https://travis-ci.org/peopleagainstprisons/earmms_keyderiv
[catflap]: https://github.com/passcod/catflap

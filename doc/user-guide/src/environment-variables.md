# Environment variables

- `RUSTUP_LOG` (default: none). Enables Rustup's "custom logging mode". In this mode,
  the verbosity of Rustup's log lines can be specified with `tracing_subscriber`'s
  [directive syntax]. For example, set `RUSTUP_LOG=rustup=DEBUG` to receive log lines
  from `rustup` itself with a maximal verbosity of `DEBUG`.

- `RUSTUP_HOME` (default: `~/.rustup` or `%USERPROFILE%/.rustup`). Sets the
  root `rustup` folder, used for storing installed toolchains and
  configuration options.

- `RUSTUP_TOOLCHAIN` (default: none). If set, will [override] the toolchain used
  for all rust tool invocations. A toolchain with this name should be installed,
  or invocations will fail. This can specify custom toolchains, installable
  toolchains, or the absolute path to a toolchain.

- `RUSTUP_DIST_SERVER` (default: `https://static.rust-lang.org`). Sets the root
  URL for downloading static resources related to Rust. You can change this to
  instead use a local mirror, or to test the binaries from the staging
  directory.

- ~~`RUSTUP_DIST_ROOT`~~ *deprecated* (default: `https://static.rust-lang.org/dist`).
  Use `RUSTUP_DIST_SERVER` instead.

- `RUSTUP_UPDATE_ROOT` (default `https://static.rust-lang.org/rustup`). Sets
  the root URL for downloading self-update.

- `RUSTUP_VERSION` (default: none). Overrides the rustup version (e.g. `1.27.1`)
  to be downloaded when executing `rustup-init.sh` or `rustup self update`.

- `RUSTUP_IO_THREADS` *unstable* (defaults to reported cpu count). Sets the
  number of threads to perform close IO in. Set to `1` to force
  single-threaded IO for troubleshooting, or an arbitrary number to override
  automatic detection.

- `RUSTUP_TRACE_DIR` *unstable* (default: no tracing). Enables tracing and
  determines the directory that traces will be written too. Traces are of the
  form PID.trace. Traces can be read by the Catapult project [tracing viewer].

- `RUSTUP_TERM_COLOR` (default: `auto`). Controls whether colored output is used in the terminal.
  Set to `auto` to use colors only in tty streams, to `always` to always enable colors,
  or to `never` to disable colors.

- `RUSTUP_UNPACK_RAM` *unstable* (default free memory or 500MiB if unable to tell, min 210MiB). Caps the amount of
  RAM `rustup` will use for IO tasks while unpacking.

- `RUSTUP_NO_BACKTRACE`. Disables backtraces on non-panic errors even when
  `RUST_BACKTRACE` is set.

- `RUSTUP_PERMIT_COPY_RENAME` *unstable*. When set, allows rustup to fall-back
  to copying files if attempts to `rename` result in cross-device link
  errors. These errors occur on OverlayFS, which is used by [Docker][dc]. This
  feature sacrifices some transactions protections and may be removed at any
  point. Linux only.

- `RUSTUP_AUTO_INSTALL` (default: 1) When set to `1`, installs the active
  toolchain when it is absent. Set this value to `0` to disable automatic
  installation.

- `RUSTUP_HARDLINK_PROXIES` *unstable*. When set, rustup will not attempt to
  symlink proxies and instead always use hardlinks. If you find this fixes
  a problem, then please report the issue on the [rustup issue tracker].

- `RUSTUP_TERM_PROGRESS_WHEN` (defaults: `auto`). Controls whether progress bars are shown or not.
  Set to `always` to always enable progress bars, and to `never` to disable them.

- `RUSTUP_TERM_WIDTH` (default: none). Allows to override the terminal width for progress bars.

[directive syntax]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives
[dc]: https://docs.docker.com/storage/storagedriver/overlayfs-driver/#modifying-files-or-directories
[override]: overrides.md
[tracing viewer]: https://github.com/catapult-project/catapult/blob/master/tracing/README.md
[rustup issue tracker]: https://github.com/rust-lang/rustup/issues

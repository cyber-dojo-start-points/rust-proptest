#!/bin/bash

# --------------------------------------------------------------
# Every text file here is returned to your kata after a [test] press. cargo
# writes Cargo.lock itself, from Cargo.toml, on every press, so an edit you
# made to it would be overwritten by the next one; it is removed here rather
# than handed back to you.
function cyber_dojo_exit()
{
  rm -f Cargo.lock
}
trap cyber_dojo_exit EXIT SIGTERM

# --------------------------------------------------------------
# There is no internet access here, so cargo must not go looking for proptest
# on crates.io. Everything it needs is already in this container.
export PATH=/usr/local/cargo/bin:${PATH}

# Resolve against the copy of the crates.io registry in this container, and
# say so plainly rather than reaching for a network that is not there.
export CARGO_NET_OFFLINE=true
# proptest and everything below it, compiled once when this container's image
# was built. Reusing that is what makes a press quick; without it every press
# would compile proptest from source.
export CARGO_TARGET_DIR=/rust/target-cache
# Incremental compilation saves state for a later build to read back. Every
# press gets a new container, so there is never a later build here to read it.
export CARGO_INCREMENTAL=0
# proptest saves a failing value to a file so that a later run can try it
# first. That file would arrive in your kata, and the press after it starts in
# a new container which cannot see it, so there is nothing for it to do here.
export PROPTEST_DISABLE_FAILURE_PERSISTENCE=1

# --------------------------------------------------------------
# cargo compiles src/lib.rs, whatever that file reaches through a `mod` line,
# and each .rs file directly inside tests/. Any other .rs file it never looks
# at, so one you are partway through writing could sit here failing to compile
# while your tests still pass. Each of those files is compiled on its own
# below, so that cannot happen.
#
# Such a file is compiled alone, without the rest of your crate around it, so
# one that says `crate::` or `super::` before you have added its `mod` line is
# reported here. Adding that line is what fixes it.
#
# A test only runs when cargo can see it, which means putting it directly in
# tests/. A test file in a directory below tests/ is compiled here, so it
# cannot sit there broken, but its tests do not run.
shopt -s globstar nullglob

# True when some .rs file declares ${1} as a module, which is what makes cargo
# compile the file of that name.
declared_as_a_module()
{
  local -r name="${1}"
  # busybox grep has none of the long spellings: -q is quiet, -s stays quiet
  # about a file it cannot open, -E takes an extended regexp. /dev/null gives
  # grep a file to read when you have no .rs files at all; with no file at all
  # it would sit waiting on its input.
  grep -q -s -E "^[[:space:]]*(pub[[:space:]]+)?mod[[:space:]]+${name}[[:space:]]*;" **/*.rs /dev/null
}

# True when cargo compiles ${1}, either as part of your library or as a test
# program of its own.
compiled_by_cargo()
{
  local -r file="${1}"
  case "${file}" in
    src/lib.rs) return 0 ;;  # the root of your library
    tests/*/*)  ;;           # below tests/, where cargo does not look
    tests/*.rs) return 0 ;;  # directly in tests/, so a test program of its own
  esac
  # A directory becomes a module through its mod.rs, so that file is declared
  # under the name of the directory holding it rather than its own.
  local name="$(basename "${file}" .rs)"
  if [ "${name}" = 'mod' ]; then
    name="$(basename "$(dirname "${file}")")"
  fi
  declared_as_a_module "${name}"
}

UNSEEN_FILES=()
for file in **/*.rs; do
  if ! compiled_by_cargo "${file}"; then
    UNSEEN_FILES+=("${file}")
  fi
done

# None of this runs when cargo compiles every file, which is the ordinary
# case, so an ordinary press pays nothing for it.
if [ ${#UNSEEN_FILES[@]} -gt 0 ]; then
  mkdir -p /tmp/check

  RUSTC_OPTS=()
  RUSTC_OPTS+=(--edition 2024)        # the edition Cargo.toml names
  RUSTC_OPTS+=(--emit metadata)       # report the errors, do not build a binary
  RUSTC_OPTS+=(--out-dir /tmp/check)  # nothing written there reaches your kata
  # proptest, and the crates it is built on, so that a test file compiled on
  # its own can still use proptest. Naming the one file is not enough: rustc
  # follows it to the crates below it, and looks for those in this directory.
  RUSTC_OPTS+=(-L dependency=/rust/target-cache/debug/deps)
  RUSTC_OPTS+=(--extern proptest="$(echo /rust/target-cache/debug/deps/libproptest-*.rlib)")

  # Your library, compiled on its own first, so that a test file checked below
  # can use it the same way a test file in tests/ does. It is compiled here
  # rather than taken from what cargo built, so that the check sees your
  # library as you have just written it.
  if [ -f src/lib.rs ]; then
    rustc "${RUSTC_OPTS[@]}" --crate-type lib --crate-name hiker src/lib.rs || exit 1
    RUSTC_OPTS+=(--extern hiker=/tmp/check/libhiker.rmeta)
  fi

  # --test is what makes rustc compile the #[test] functions in a file.
  # proptest! writes each of its tests as a #[test] function, and an ordinary
  # build leaves those out, so without this the body of a property test would
  # go unread and a broken one would look fine.
  for file in "${UNSEEN_FILES[@]}"; do
    rustc "${RUSTC_OPTS[@]}" --test "${file}" || exit 1
  done
fi

# --------------------------------------------------------------
# proptest runs each test many times over, with values it chooses itself, and
# when one of them fails it shrinks that value to the smallest which still
# fails and reports that one. tests/hiker_tests.rs shows the shape to copy.
CARGO_TEST_OPTS=()
CARGO_TEST_OPTS+=(--features strict)  # treat a compiler warning as an error

cargo test "${CARGO_TEST_OPTS[@]}" -- --nocapture

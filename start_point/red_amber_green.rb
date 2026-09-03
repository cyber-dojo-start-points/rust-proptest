
# An assertion that did not hold is a test failing, which is red. Anything
# else that stopped a test is amber: an index out of range, an unwrap of a
# None, arithmetic that overflowed, a panic! of your own. cargo test reports
# both alike as FAILED, so the two are told apart by the panic message rather
# than by the summary.

lambda { |stdout,stderr,status|
  output = stdout + stderr

  # What rust prints on the line after naming where a panic happened. proptest
  # reports the value it shrank to on the same kind of line, prefixed with
  # "Test failed: ", so dropping that prefix leaves the message itself either
  # way.
  panics = output.scan(/panicked at [^\n]*\n([^\n]*)/).flatten
  panics = panics.map { |message| message.sub(/\ATest failed: /, '') }

  # std writes "assertion `left == right` failed" for assert_eq! and
  # "assertion failed: ..." for assert!, and proptest writes "assertion
  # failed: `(left == right)`" for prop_assert_eq!. All three begin the same
  # way, and a message that does not begin that way is not an assertion, so a
  # panic this does not recognise stays amber rather than turning red.
  assertion = /\Aassertion (failed|`)/

  if /test result: FAILED/.match(output)
    return :amber if panics.any? { |message| !assertion.match(message) }
    return :red
  end

  # A summary that counted at least one passing test. cargo prints one summary
  # per test program, and a kata whose tests have all been deleted still gets
  # one for the library and one for the doc-tests, both counting nothing. So a
  # green needs a summary which actually ran something, not merely an ok.
  return :green if status == 0 && /test result: ok\. [1-9]\d* passed/.match(output)

  # Nothing ran at all: a file that would not compile, or one of the files
  # cargo does not see failing the check cyber-dojo.sh makes of it.
  return :amber
}

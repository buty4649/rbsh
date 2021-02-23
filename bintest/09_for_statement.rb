require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")

assert("for_statement") do
  assert_stdout("test\n", "for test in test; do echo $test; done")
  assert_stdout("test\n", "for test in test; do echo $test; end")
  assert_stdout("test\n", "for test in test; { echo $test; }")
  assert_stdout("test\n", "for test in test; echo $test; end")

  assert_stdout("foo\nbar\n", "for test in foo bar; echo $test; end")

  assert_stdout("test\n", ["for test in test", "echo $test", "end"])
end

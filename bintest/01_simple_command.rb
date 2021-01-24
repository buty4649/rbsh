require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")
require 'tempfile'

assert('simple command') do
  ENV['REDDISH_BINTEST_ENV'] = "test"

  [
    "echo test",
    "echo 'test'",
    'echo "test"',
    %q|echo t"es"'t'|,
    'echo $REDDISH_BINTEST_ENV',
    'echo "$REDDISH_BINTEST_ENV"',
    'echo %Q!test!',
    'echo %Q(test)',
    'echo %Q[test]',
    'echo %Q{test}',
    'echo %Q<test>',
    'echo %Q|test|',
    'echo %Q|$REDDISH_BINTEST_ENV|',
    'echo %!test!',
    'echo %!$REDDISH_BINTEST_ENV!',
    "echo test &",
    "echo test ;",
    ["echo \\", "test"],
  ].each do |command|
    assert_stdout("test\n", command)
  end
  assert_stdout("\'\n", %q{echo '\''})
  assert_stdout("$REDDISH_BINTEST_ENV\n", %Q{echo '$REDDISH_BINTEST_ENV'})
  assert_stdout("$REDDISH_BINTEST_ENV\n", 'echo %q|$REDDISH_BINTEST_ENV|')
  assert_stdout("testtesttest\n", %q|TEST=test"test"'test' bash -c 'echo $TEST'|)
  assert_stdout("test\n", %q|TEST=test bash -c "echo \$TEST"|)
  ENV.delete('REDDISH_BINTEST_ENV')

  assert_equal("OK\n", `echo "echo OK" | #{BIN_PATH}`, "pipe exec")
  assert_equal("OK\n", `#{BIN_PATH} -c "echo OK"`, "-c option")

  Tempfile.open do |fp|
    fp.write(<<~EOS)
    #!#{BIN_PATH}

    echo OK
    EOS
    fp.flush
    assert_equal("OK\n", `#{BIN_PATH} #{fp.path}`, "script file")
  end
end

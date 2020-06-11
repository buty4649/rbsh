require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")

assert('simple command') do
  assert_equal("test\n", run('echo test').stdout)
  assert_equal("test\n", run('echo "test"').stdout)
  assert_equal("test\n", run("echo 'test'").stdout)
  assert_equal("testtesttest\n", run(%Q{echo "test"test'test'}).stdout)

  ENV['REDDISH_BINTEST_ENV'] = "test"

  assert_equal("test\n", run('echo $REDDISH_BINTEST_ENV').stdout)
  assert_equal("test\n", run('echo "$REDDISH_BINTEST_ENV"').stdout)
  assert_equal("$REDDISH_BINTEST_ENV\n", run(%Q{echo '$REDDISH_BINTEST_ENV'}).stdout)

  assert_equal("test\n", run('echo %Q!test!').stdout)
  assert_equal("test\n", run('echo %Q(test)').stdout)
  assert_equal("test\n", run('echo %Q[test]').stdout)
  assert_equal("test\n", run('echo %Q{test}').stdout)
  assert_equal("test\n", run('echo %Q<test>').stdout)
  assert_equal("test\n", run('echo %Q|test|').stdout)
  assert_equal("test\n", run('echo %Q|$REDDISH_BINTEST_ENV|').stdout)

  assert_equal("$REDDISH_BINTEST_ENV\n", run('echo %q|$REDDISH_BINTEST_ENV|').stdout)

  assert_equal("test\n", run('echo %!test!').stdout)
  assert_equal("test\n", run('echo %!$REDDISH_BINTEST_ENV!').stdout)

  assert_equal("test\n", run("echo test &").stdout)
  assert_equal("test\n", run("echo test ;").stdout)
  assert_equal("test\n", run("echo test &;").stdout)
  assert_equal("test\n", run("echo test & ;").stdout)

  ENV.delete('REDDISH_BINTEST_ENV')
end

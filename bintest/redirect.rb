require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")
require 'tempfile'

assert('redirect') do
  tempfile = Tempfile.new

  assert_equal("", run("echo test > #{tempfile.path}").stdout)
  assert_equal("test\n", File.read(tempfile.path))

  assert_equal("test\n", run("cat < #{tempfile.path}").stdout)

  assert_equal("", run("echo test >> #{tempfile.path}").stdout)
  assert_equal("test\ntest\n", File.read(tempfile.path))

  tempfile.close
end

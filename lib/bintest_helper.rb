require 'open3'
require 'json'

BIN_PATH = File.join(File.dirname(__FILE__), "../mruby/bin/reddish")
FDTEST_PATH = File.join(File.dirname(__FILE__), "../mruby/build/fdtest/bin/fdtest")
SIGTEST_PATH = File.join(File.dirname(__FILE__), "../mruby/build/sigtest/bin/sigtest")

def run(command)
  o, e = Open3.popen3(BIN_PATH) do |stdin, stdout, stderr, wait|
    if command.class == String
      stdin.puts(command)
    else
      command.each {|cmd| stdin.puts(cmd) }
    end
    stdin.puts("exit")
    [stdout.read, stderr.read]
  end
  Struct.new(:stdout, :stderr).new(o, e)
end

def assert_stdout(assert, command)
  assert_equal(assert, run(command).stdout, command)
end

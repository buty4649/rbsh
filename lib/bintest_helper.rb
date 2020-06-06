require 'open3'
require 'json'

BIN_PATH = File.join(File.dirname(__FILE__), "../mruby/bin/reddish")
FDTEST_PATH = File.join(File.dirname(__FILE__), "../mruby/build/fdtest/bin/fdtest")
SIGTEST_PATH = File.join(File.dirname(__FILE__), "../mruby/build/sigtest/bin/sigtest")

def run(command)
  o, e, s = Open3.capture3(BIN_PATH, :stdin_data => command)
  Struct.new(:stdout, :stderr, :status).new(o, e, s)
end

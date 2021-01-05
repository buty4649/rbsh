require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")
require 'tempfile'
require 'pty'
require 'expect'

def sigtest(command)
  PTY.spawn("#{BIN_PATH} -i") do |pty_out, pty_in, pid|
    pty_out.expect(/^reddish> /, 10)
    pty_in.puts(command)
    pty_in.puts("exit")
    pty_out.expect(/^sigtest wait/, 10)
    Process.kill(:INT, pid)
    Process.waitpid(pid)
  end
end

assert('jobcontrol') do
  tf = Tempfile.new
  tp = tf.path

  sigtest("#{SIGTEST_PATH} #{tp}")
  assert_equal("SIGINT\n", File.read(tp))

  File.truncate(tp, 0)
  sigtest("#{SIGTEST_PATH} #{tp} | #{SIGTEST_PATH} #{tp}")
  assert_match("SIGINT*SIGINT*", File.read(tp))
  tf.close
end

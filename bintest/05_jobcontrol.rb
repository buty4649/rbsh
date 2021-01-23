require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")
require 'tempfile'
require 'expect'

def sigtest(command)
  Open3.popen3(BIN_PATH) do |stdin, stdout, stderr, wait_thr|
    stdin.puts(command)
    stdin.puts("exit")
    stdout.expect(/^sigtest wait/, 10)
    Process.kill(:INT, wait_thr.pid)
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

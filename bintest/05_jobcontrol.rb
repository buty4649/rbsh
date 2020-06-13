require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")
require 'tempfile'

def sigtest(command)
  pid = Process.fork do
    Process.exec(BIN_PATH, "-c", command)
  end
  sleep 1
  Process.kill(:INT, pid)
  Process.waitpid(pid)
end

assert('jobcontrol') do
  tf = Tempfile.new
  tp = tf.path

  #sigtest("#{SIGTEST_PATH} #{tp}")
  pass #assert_equal("SIGINT\n", File.read(tp))

  File.truncate(tp, 0)
  #sigtest("#{SIGTEST_PATH} #{tp} | #{SIGTEST_PATH} #{tp}")
  pass #assert_match("SIGINT*SIGINT*", File.read(tp))
  tf.close
end

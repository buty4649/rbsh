require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")
require 'tempfile'
require 'expect'

def sigtest(command)
  Open3.popen3(BIN_PATH) do |stdin, stdout, stderr, wait_thr|
    stdin.puts(command)
    stdin.close
    stdout.expect(/^sigtest wait/, 10)
    Process.kill(:INT, wait_thr.pid)
  end
end

assert('jobcontrol') do
  Tempfile.open do |tempfile|
    tp = tempfile.path

    sigtest("#{SIGTEST_PATH} #{tp}")
    assert_equal("SIGINT\n", File.read(tp))

    File.truncate(tp, 0)
    sigtest("#{SIGTEST_PATH} #{tp} | #{SIGTEST_PATH} #{tp}")
    assert_match("SIGINT*SIGINT*", File.read(tp))

    File.truncate(tp, 0)
    sigtest("while true; #{SIGTEST_PATH} #{tp}; end")
    assert_match("SIGINT\n", File.read(tp))

    File.truncate(tp, 0)
    sigtest("for FOO in BAR; #{SIGTEST_PATH} #{tp}; end")
    assert_match("SIGINT\n", File.read(tp))
  end
end

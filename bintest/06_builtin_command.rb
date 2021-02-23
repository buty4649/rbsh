require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")
require 'tempfile'

assert('builtin command call') do
  assert_equal("test\n", run("builtin echo test").stdout)
  assert_equal("reddish: notfound: Not a shell builtin\n", run("builtin notfound").stderr)

  Tempfile.open do |fp|
    run("builtin echo test > #{fp.path}")
    assert_equal("test\n", fp.read)
  end
end

assert('cd') do
  Dir.mktmpdir do |dir|
    assert_equal(dir, run("cd #{dir}; pwd").stdout.chomp)
    assert_equal(dir, run("HOME=#{dir} cd; pwd").stdout.chomp)

    current_dir = Dir.pwd
    assert_equal(current_dir, run("cd #{dir}; cd -;pwd").stdout.chomp)
  end
end

assert('echo') do
  assert_equal("test\n", run("echo test").stdout)
  assert_equal("test", run("echo -n test").stdout)
  assert_equal("\e\n", run('echo -e \e').stdout)
  assert_equal("\e",   run('echo -ne \e').stdout)
  assert_equal("\e\n", run('echo -e \033').stdout)
  assert_equal("\e\n", run('echo -e \x1b').stdout)
  assert_equal("イ\n", run('echo -e \u30a4').stdout)
  assert_equal("🍣\n", run('echo -e \U1f363').stdout)
  assert_equal("🍣🍣\n", run('echo -e \u{1f363 1f363}').stdout)
  assert_equal("\\e\n", run('echo -E \e').stdout)
  assert_equal("\\e", run('echo -nE \e').stdout)
  assert_equal("-help\n", run('echo -help').stdout)
end

assert('puts') do
  assert_equal("test\n", run("puts test").stdout)
  assert_equal("\e\n", run("puts \e").stdout)
  assert_equal("test\ntest\n", run("puts test test").stdout)
end

assert('read') do
  ENV.delete('TEST')

  assert_stdout("test\n", ["read TEST; echo $TEST", "test"])

  assert_stdout("foo\n", ["read TEST1 TEST2 TEST3; echo $TEST1", "foo bar"])
  assert_stdout("bar\n", ["read TEST1 TEST2 TEST3; echo $TEST2", "foo bar"])
  assert_stdout("\n",    ["read TEST1 TEST2 TEST3; echo $TEST3", "foo bar"])
end

assert("iruby") do
  assert_stdout("OK\n", %|iruby -e 'puts "OK"'|)
  assert_stdout("iruby use mruby ?.?.? *", 'iruby -v')

  Tempfile.open do |tempfile|
    tempfile.puts(%|puts "OK"|)
    tempfile.close

    tp = tempfile.path

    assert_stdout("OK\n", %|iruby #{tp}|)

    cmd = "#{BIN_PATH} -r #{tp}"
    assert_equal("OK\n", `#{cmd}`, cmd)
  end
end

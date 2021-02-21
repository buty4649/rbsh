require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")

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

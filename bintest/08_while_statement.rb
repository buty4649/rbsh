require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")

assert("while_statement") do
  expect_output = (0...10).to_a.join("\n") + "\n"
  [
    "while [ $TEST -ne 10 ]; echo $TEST; TEST=`expr $TEST + 1`; end",
    "while [ $TEST -ne 10 ]; do echo $TEST; TEST=`expr $TEST + 1`; done",
    "while [ $TEST -ne 10 ]; do echo $TEST; TEST=`expr $TEST + 1`; end",
  ].each do |command|
    ENV['TEST'] = "0"
    assert_stdout(expect_output, command)
  end

  ["while", "do", "done"].each do |keyword|
    assert_stdout("#{keyword}\n", "echo #{keyword}")
  end

  ENV['TEST'] = "0"
  assert_stdout("OK\n#{expect_output}", "echo OK; while [ $TEST -ne 10 ]; echo $TEST; TEST=`expr $TEST + 1`; end")

  Tempfile.open do |tempfile|
    tp = tempfile.path
    ENV['TEST'] = "0"
    command = "while [ $TEST -ne 10 ]; echo $TEST; TEST=`expr $TEST + 1`; end > #{tp}"
    run(command)
    assert_equal(expect_output, File.read(tp), command)
  end

  ENV['TEST'] = "0"
  assert_stdout(expect_output,  "while [ $TEST -ne 10 ]; echo $TEST; TEST=`expr $TEST + 1`; end | cat")

  ENV['TEST'] = "0"
  assert_stdout(expect_output,  ["while [ $TEST -ne 10 ]", "echo $TEST", "TEST=`expr $TEST + 1`", "end"])

  ENV.delete("TEST")
end

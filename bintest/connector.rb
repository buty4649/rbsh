require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")

assert('connector') do
  assert_equal("1\n2\n3\n", run("echo 1 && echo 2 && echo 3").stdout, "true && true && true")
  assert_equal("1\n", run("echo 1 && false && echo 3").stdout,  "true && false && true")
  assert_equal("",    run("false && echo 2 && echo 3").stdout,  "false && true && true")

  assert_equal("1\n", run("echo 1 || echo 2 || echo 3").stdout, "true || true || true")
  assert_equal("2\n", run("false  || echo 2 || echo 3").stdout, "false || true || true")

  assert_equal("1\n2\n", run("echo 1 && echo 2 || echo 3").stdout, "true && true || true")
  assert_equal("1\n3\n", run("echo 1 && false  || echo 3").stdout, "true && false|| true")

  assert_equal("2\n3\n", run("false || echo 2 && echo 3").stdout, "false || true && true")
end

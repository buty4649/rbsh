require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")
require 'tempfile'

def fdtest_run(args)
  result = run("#{FDTEST_PATH} #{args}")
  out = result.stdout || result.stderr
  out.empty? ? {} : JSON.parse(out)
end

def json_from_file(filename)
  buffer = File.read(filename)
  buffer.empty? ? {} : JSON.parse(buffer)
end

assert('redirect') do
  tempfile = Tempfile.new
  tp = tempfile.path

  assert_equal(tp, fdtest_run("< #{tp}")["0"],       "<")
  assert_equal(tp, fdtest_run("3<  #{tp}")["3"],     "n<")
  assert_equal(nil,fdtest_run("<&-")["0"],           "<&-")
  assert_equal(tp, fdtest_run("3< #{tp} <&3")["0"],  "<&n")
  assert_equal(nil,fdtest_run("3< #{tp} <&3-")["3"], "<&n-")
  assert_equal(nil,fdtest_run("3< #{tp} 3<&-")["3"], "n<&-")
  assert_equal(tp, fdtest_run("3< #{tp} 4<&3")["3"], "n<&n")
  assert_equal(tp, fdtest_run("3< #{tp} 4<&3")["4"], "n<&n")
  assert_equal(nil,fdtest_run("3< #{tp} 4<&3-")["3"],"n<&n-")
  assert_equal(tp ,fdtest_run("3< #{tp} 4<&3-")["4"],"n<&n-")

  assert_equal(nil,fdtest_run("> #{tp}")["1"],       ">")
  assert_equal(tp, json_from_file(tp)["1"],   ">")
  assert_equal(tp, fdtest_run("3> #{tp}")["3"],      "n>")
  assert_equal(nil,fdtest_run(">&-")["1"],           ">&-")
  assert_equal(nil,fdtest_run("3> #{tp} >&3")["1"],  ">&n")
  assert_equal(tp, json_from_file(tp)["1"],   ">&n")
  assert_equal(tp, json_from_file(tp)["3"],   ">&n")
  assert_equal(nil,fdtest_run(">&2-")["2"],          ">&n-")
  assert_equal(nil,fdtest_run("2>&-")["2"],          "n>&-")
  assert_equal(tp, fdtest_run("3> #{tp} 4>&3")["3"], "n>&n")
  assert_equal(tp, fdtest_run("3> #{tp} 4>&3")["4"], "n>&n")
  assert_equal(nil,fdtest_run("3> #{tp} 4>&3-")["3"],"n>&n-")
  assert_equal(tp, fdtest_run("3> #{tp} 4>&3-")["4"],"n>&n-")
  assert_equal(nil,fdtest_run("&> #{tp}")["1"],       "&>")
  assert_equal(tp, json_from_file(tp)["1"],    "&>")
  assert_equal(tp, json_from_file(tp)["2"],    "&>")
  assert_equal(nil,fdtest_run(">& #{tp}")["1"],       ">&")
  assert_equal(tp, json_from_file(tp)["1"],    ">&")
  assert_equal(tp, json_from_file(tp)["2"],    ">&")

  run("echo test >  #{tp}")
  run("echo test >> #{tp}")
  assert_equal("test\ntest\n", File.read(tp), ">>")
  run("echo test 2>> #{tp} >&2")
  assert_equal("test\ntest\ntest\n", File.read(tp), "n>>")

  result = run("3> #{tp} #{FDTEST_PATH}")
  out = result.stdout || result.stderr
  assert_equal(tp, (out.empty? ? {} : JSON.parse(out))["3"], "3> file fdtest")

  fdtest_run("3> #{tp} #{tp}")
  assert_equal(tp, json_from_file(tp)["3"], "fdtest 3> file file")

  tempfile.close

  assert_equal(tp, fdtest_run("<> #{tp}")["0"], "<>")
  assert_true(File.exist?(tp), "<>")
  FileUtils.rm(tp)

  assert_equal(tp, fdtest_run("3<> #{tp}")["3"], "n<>")
  assert_true(File.exist?(tp), "3<>")
  FileUtils.rm(tp)
end

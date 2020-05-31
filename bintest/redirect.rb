require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")
require 'tempfile'
require 'json'

def fdtest_run(args)
  fdtest = File.join(File.dirname(__FILE__), "../mruby/build/fdtest/bin/fdtest")
  out = run("#{fdtest} #{args}").stdout
  if out == ""
    # If STDOUT is closed, STDERR will be used.
    out = run("#{fdtest} #{args}").stderr
  end
  out == "" ? {} : JSON.parse(out)
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
  assert_equal(tp, JSON.parse(File.read(tp))["1"],   ">")
  assert_equal(tp, fdtest_run("3> #{tp}")["3"],      "n>")
  assert_equal(nil,fdtest_run(">&-")["1"],           ">&-")
  assert_equal(nil,fdtest_run("3> #{tp} >&3")["1"],  ">&n")
  assert_equal(tp, JSON.parse(File.read(tp))["1"],   ">&n")
  assert_equal(tp, JSON.parse(File.read(tp))["3"],   ">&n")
  assert_equal(nil,fdtest_run(">&2-")["2"],          ">&n-")
  assert_equal(nil,fdtest_run("2>&-")["2"],          "n>&-")
  assert_equal(tp, fdtest_run("3> #{tp} 4>&3")["3"], "n>&n")
  assert_equal(tp, fdtest_run("3> #{tp} 4>&3")["4"], "n>&n")
  assert_equal(nil,fdtest_run("3> #{tp} 4>&3-")["3"],"n>&n-")
  assert_equal(tp, fdtest_run("3> #{tp} 4>&3-")["4"],"n>&n-")
  assert_equal(nil,fdtest_run("&> #{tp}")["1"],       "&>")
  assert_equal(tp, JSON.parse(File.read(tp))["1"],    "&>")
  assert_equal(tp, JSON.parse(File.read(tp))["2"],    "&>")
  assert_equal(nil,fdtest_run(">& #{tp}")["1"],       ">&")
  assert_equal(tp, JSON.parse(File.read(tp))["1"],    ">&")
  assert_equal(tp, JSON.parse(File.read(tp))["2"],    ">&")

  run("echo test >  #{tp}")
  run("echo test >> #{tp}")
  assert_equal("test\ntest\n", File.read(tp), ">>")
  run("echo test 2>> #{tp} >&2")
  assert_equal("test\ntest\ntest\n", File.read(tp), "n>>")

  tempfile.close

  assert_equal(tp, fdtest_run("<> #{tp}")["0"], "<>")
  assert_true(File.exist?(tp), "<>")
  FileUtils.rm(tp)

  assert_equal(tp, fdtest_run("3<> #{tp}")["3"], "n<>")
  assert_true(File.exist?(tp), "3<>")
  FileUtils.rm(tp)
end

require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")

assert("if_statement") do
  [
    "if true; then echo OK; fi",
    "if true; then echo OK; end",
    "if false;then echo NG; else echo OK; fi",
    "if false;then echo NG; else echo OK; end",
    "if false;then echo NG; elif true; then echo OK; fi",
    "if false;then echo NG; elif true; then echo OK; end",
    "if false;then echo NG; elif false; then echo NG; else echo OK; fi",
    "if false;then echo NG; elif false; then echo NG; else echo OK; end",
    "if false;then echo NG; elif false; then echo NG; elif true; then echo OK; fi",
    "if false;then echo NG; elif false; then echo NG; elif true; then echo OK; end",
    "if false;then echo NG; elif false; then echo NG; elsif true; then echo OK; fi",
    "if false;then echo NG; elif false; then echo NG; elsif true; then echo OK; end",
    "if false;then echo NG; elsif true; then echo OK; fi",
    "if false;then echo NG; elsif true; then echo OK; end",
    "if false;then echo NG; elsif false; then echo NG; else echo OK; fi",
    "if false;then echo NG; elsif false; then echo NG; else echo OK; end",
    "if false;then echo NG; elsif false; then echo NG; elif true; then echo OK; fi",
    "if false;then echo NG; elsif false; then echo NG; elif true; then echo OK; end",
    "if false;then echo NG; elsif false; then echo NG; elsif true; then echo OK; fi",
    "if false;then echo NG; elsif false; then echo NG; elsif true; then echo OK; end",
    "if true; then if true; then echo OK; fi; fi",
    "if true && true; then echo OK; fi",
    "if false; true; then echo OK; fi",
    "if true; then echo OK; fi &",
    "unless false; then echo OK; end",
    "unless true; then echo NG; else echo OK; end",
    "if true; then unless false; then echo OK; end; fi",
    "unless false; then if true; then echo OK; fi; end",
  ].each do |command|
    assert_equal("OK\n", run(command)[0], command)
  end

  assert_equal("if\n", run("echo if")[0], "echo if")

  command = "echo OK; if true; then echo OK; fi"
  assert_equal("OK\nOK\n", run(command)[0], command)
end

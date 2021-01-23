require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")

assert("if_statement") do
  [
    "if true; echo OK; end",
    "if false; echo NG; else echo OK; end",
    "if false; echo NG; elsif true; echo OK; end",
    "if false; echo NG; elsif false; echo NG; else echo OK; end",
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
    "unless false; echo OK; end",
    "unless true; echo NG; else echo OK; end",
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

  Tempfile.open do |tempfile|
    tp = tempfile.path
    command = "if true; then echo OK; fi > #{tp}"
    run(command)
    assert_equal("OK\n", File.read(tp), command)

    Tempfile.open do |tempfile2|
      tp2 = tempfile2.path
      command = "if true; then if true; then echo tp2; fi > #{tp2}; echo tp; fi > #{tp}"
      run(command)
      assert_equal("tp\n", File.read(tp), command)
      assert_equal("tp2\n", File.read(tp2), command)
    end
  end

  command = "if true; then echo FOO; fi && echo BAR"
  assert_equal("FOO\nBAR\n", run(command)[0], command)

  command = "if true; then echo FOO; false; fi && echo BAR"
  assert_equal("FOO\n", run(command)[0], command)

  command = "if false; then echo NG; fi || echo OK"
  assert_equal("OK\n", run(command)[0], command)

  command = "if true; then echo OK; fi | cat"
  assert_equal("OK\n", run(command)[0], command)
end

require File.join(File.dirname(__FILE__), "../lib/bintest_helper.rb")
require 'tempfile'
require 'json'

class Tempfiles
  def initialize
    @files = [Tempfile.new, Tempfile.new, Tempfile.new]
    @paths = @files.map(&:path)
  end

  def [](i)
    @paths[i]
  end

  def read(num)
    JSON.parse(File.read(@paths[num]))
  end

  def close
    @files.each(&:close)
  end
end

assert('pipe') do
  tf = Tempfiles.new

  run("#{FDTEST_PATH} #{tf[0]} | #{FDTEST_PATH} #{tf[1]} | #{FDTEST_PATH} #{tf[2]}")
  assert_equal(tf.read(0)[1], tf.read(1)[0])
  assert_equal(tf.read(1)[1], tf.read(2)[0])

  assert_equal("1\n", run("#{FDTEST_PATH} #{tf[0]} | #{FDTEST_PATH} #{tf[1]} && echo 1").stdout)
  assert_equal(tf.read(0)[1], tf.read(1)[0])
  assert_equal("", run("#{FDTEST_PATH} #{tf[0]} | #{FDTEST_PATH} #{tf[1]} || echo 1").stdout)
  assert_equal(tf.read(0)[1], tf.read(1)[0])

  assert_equal("1\n", run("echo 1 && #{FDTEST_PATH} #{tf[0]} | #{FDTEST_PATH} #{tf[1]}").stdout)
  assert_equal(tf.read(0)[1], tf.read(1)[0])
  assert_equal("", run("false || #{FDTEST_PATH} #{tf[0]} | #{FDTEST_PATH} #{tf[1]}").stdout)
  assert_equal(tf.read(0)[1], tf.read(1)[0])

  tf.close
end

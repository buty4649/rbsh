require 'fileutils'

MRUBY_VERSION="2.1.0"

file :mruby do
  #sh "git clone --depth=1 https://github.com/mruby/mruby"
  sh "curl -L --fail --retry 3 --retry-delay 1 https://github.com/mruby/mruby/archive/#{MRUBY_VERSION}.tar.gz -s -o - | tar zxf -"
  FileUtils.mv("mruby-#{MRUBY_VERSION}", "mruby")
end

APP_NAME=ENV["APP_NAME"] || "reddish"
APP_ROOT=ENV["APP_ROOT"] || Dir.pwd
# avoid redefining constants in mruby Rakefile
mruby_root=File.expand_path(ENV["MRUBY_ROOT"] || "#{APP_ROOT}/mruby")
mruby_config=File.expand_path(ENV["MRUBY_CONFIG"] || "build_config.rb")
ENV['MRUBY_ROOT'] = mruby_root
ENV['MRUBY_CONFIG'] = mruby_config
Rake::Task[:mruby].invoke unless Dir.exist?(mruby_root)
Dir.chdir(mruby_root)
load "#{mruby_root}/Rakefile"

desc "compile binary"
task :compile => [:all] do
  bindir = File.join(APP_ROOT, "bin")
  FileUtils.mkdir_p(bindir)
  %W(#{mruby_root}/bin/#{APP_NAME}).each do |bin|
    next unless File.exist?(bin)
    #sh "strip --strip-unneeded #{bin}"
    FileUtils.cp(bin, File.join(bindir, "#{APP_NAME}"))
  end
end

namespace :test do
  desc "run mruby & unit tests"
  # only build mtest for host
  task :mtest => :compile do
    # in order to get mruby/test/t/synatx.rb __FILE__ to pass,
    # we need to make sure the tests are built relative from mruby_root
    MRuby.each_target do |target|
      # only run unit tests here
      target.enable_bintest = false
      run_test if target.test_enabled?
    end
  end

  def clean_env(envs)
    old_env = {}
    envs.each do |key|
      old_env[key] = ENV[key]
      ENV[key] = nil
    end
    yield
    envs.each do |key|
      ENV[key] = old_env[key]
    end
  end

  desc "run integration tests"
  task :bintest => :compile do
    MRuby.each_target do |target|
      clean_env(%w(MRUBY_ROOT MRUBY_CONFIG)) do
        run_bintest if target.bintest_enabled?
      end
    end
  end
end

desc "run all tests"
Rake::Task['test'].clear
task :test => ["test:mtest", "test:bintest"]

desc "cleanup"
Rake::Task['clean'].clear
task :clean do
  MRuby.each_target do |t|
    build_dir = File.join(t.build_dir, "mrbgems")
    gem_build_dirs = t.gem_dir_to_repo_url.values.map{|g| File.join(build_dir, File.basename(g))}
    gem_build_dirs << File.join(build_dir, APP_NAME)
    gem_build_dirs << File.join(build_dir, "mruby-reddish-parser")
    gem_build_dirs << File.join(build_dir, "mruby-bin-fdtest")
    FileUtils.rm_rf(gem_build_dirs, **{verbose: true, secure: true})
  end
end

def gem_config(conf)
  #conf.gembox 'default'

  # be sure to include this gem (the cli app)
  conf.gem File.expand_path(File.dirname(__FILE__))

  conf.gem 'mrbgems/mruby-io-dup2'
  conf.gem 'mrbgems/mruby-io-fcntl'
  conf.gem 'mrbgems/mruby-io-stat'
  conf.gem 'mrbgems/mruby-reddish-parser'
  conf.gem core: 'mruby-struct'
  conf.gem mgem: 'mruby-linenoise'
  conf.gem mgem: 'mruby-signal-thread'
  conf.gem mgem: 'mruby-env'
  conf.gem mgem: 'mruby-onig-regexp'
  conf.gem github: 'buty4649/mruby-process', branch: 'improve-process-exec'
  conf.gem github: 'haconiwa/mruby-exec'
end

MRuby::Build.new do |conf|
  toolchain :gcc

  conf.enable_bintest
  conf.enable_debug
  #conf.enable_test

  gem_config(conf)
end

MRuby::Build.new('fdtest') do |conf|
  toolchain :gcc

  conf.gem 'mrbgems/mruby-bin-fdtest'
  conf.gem mgem: 'mruby-dir-glob'
  conf.gem mgem: 'mruby-regexp-pcre'
end

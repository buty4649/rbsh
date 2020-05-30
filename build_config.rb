def gem_config(conf)
  #conf.gembox 'default'

  # be sure to include this gem (the cli app)
  conf.gem File.expand_path(File.dirname(__FILE__))

  conf.gem 'mrbgems/mruby-io-fcntl'
  conf.gem 'mrbgems/mruby-io-dup2'
  conf.gem 'mrbgems/mruby-reddish-parser'
  conf.gem mgem: 'mruby-linenoise'
  conf.gem mgem: 'mruby-signal-thread'
  conf.gem mgem: 'mruby-env'
  conf.gem mgem: 'mruby-regexp-pcre'
  conf.gem github: 'buty4649/mruby-process', branch: 'improve-process-exec'
  conf.gem github: 'haconiwa/mruby-exec'
end

MRuby::Build.new do |conf|
  toolchain :gcc

  conf.enable_bintest
  #conf.enable_debug
  #conf.enable_test

  cc.flags += %w[-static]
  gem_config(conf)
end

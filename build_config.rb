def gem_config(conf)
  #conf.gembox 'default'

  conf.gem 'mrbgems/mruby-io-dup2'
  conf.gem 'mrbgems/mruby-io-fcntl'
  conf.gem 'mrbgems/mruby-io-stat'
  conf.gem 'mrbgems/mruby-reddish-parser'
  conf.gem core: 'mruby-kernel-ext'
  conf.gem core: 'mruby-struct'
  conf.gem core: 'mruby-pack'
  conf.gem mgem: 'mruby-linenoise'
  conf.gem mgem: 'mruby-signal-thread'
  conf.gem mgem: 'mruby-dir'
  conf.gem mgem: 'mruby-env'
  conf.gem mgem: 'mruby-onig-regexp'
  conf.gem github: 'buty4649/mruby-process', branch: 'improve-process-exec'
  conf.gem github: 'buty4649/mruby-getopts', branch: 'add-prog-name'
  conf.gem github: 'haconiwa/mruby-exec'
end

MRuby::Build.new do |conf|
  toolchain :gcc

  conf.enable_bintest
  conf.enable_debug
  #conf.enable_test

  # be sure to include this gem (the cli app)
  conf.gem File.expand_path(File.dirname(__FILE__))
  gem_config(conf)
end

MRuby::Build.new('mirb') do |conf|
  toolchain :gcc

  conf.gem core: 'mruby-bin-mirb'
  gem_config(conf)
end

MRuby::Build.new('fdtest') do |conf|
  toolchain :gcc

  conf.gem 'mrbgems/mruby-bin-fdtest'
  conf.gem mgem: 'mruby-dir-glob'
  conf.gem mgem: 'mruby-regexp-pcre'
end

MRuby::Build.new('sigtest') do |conf|
  toolchain :gcc

  conf.gem 'mrbgems/mruby-bin-sigtest'
  conf.gem mgem: 'mruby-io'
  conf.gem mgem: 'mruby-process'
  conf.gem github: 'buty4649/mruby-signal-thread', branch: 'reset-sigmask'
end

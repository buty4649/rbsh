def gem_config(conf)
  #conf.gembox 'default'

  # be sure to include this gem (the cli app)
  conf.gem File.expand_path(File.dirname(__FILE__))

  conf.gem :git => 'https://github.com/buty4649/mruby-process', :branch => 'improve-process-exec'
  conf.gem mgem: 'mruby-linenoise'
  conf.gem mgem: 'mruby-signal-thread'
  conf.gem mgem: 'mruby-env'
  conf.gem mgem: 'mruby-regexp-pcre'
  conf.gem git: 'https://github.com/buty4649/mruby-exec', branch: 'add-execve_override_procname'
end

MRuby::Build.new do |conf|
  toolchain :gcc

  #conf.enable_bintest
  #conf.enable_debug
  #conf.enable_test

  cc.flags += %w[-static]
  gem_config(conf)
end

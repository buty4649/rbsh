MRuby::Build.new do |conf|
  conf.toolchain

  conf.gembox 'full-core'
  conf.gem 'mruby/mrbgems/mruby-signal-exception'

  conf.enable_debug
  conf.enable_bintest
  conf.enable_test
end

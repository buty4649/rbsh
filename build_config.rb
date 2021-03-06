MRuby::Build.new do |conf|
  toolchain :gcc

  conf.build_mrbc_exec
  conf.enable_bintest
  conf.enable_debug
  #conf.enable_test
  conf.disable_presym

  # be sure to include this gem (the cli app)
  dir = File.expand_path(File.dirname(__FILE__))
  conf.gem dir

  conf.gembox "stdlib"
  conf.gembox "stdlib-ext"
  conf.gembox "stdlib-io"
  conf.gembox "math"
  conf.gem core: "mruby-eval"
  conf.gem core: "mruby-metaprog"
  conf.gem core: "mruby-method"
  conf.gem core: 'mruby-catch'
end

MRuby::Build.new('fdtest') do |conf|
  toolchain :gcc
  conf.disable_presym

  conf.gem 'mrbgems/mruby-bin-fdtest'
end

MRuby::Build.new('sigtest') do |conf|
  toolchain :gcc
  conf.disable_presym

  conf.gem 'mrbgems/mruby-bin-sigtest'
end

MRuby::Gem::Specification.new('reddish') do |spec|
  spec.license = 'MIT'
  spec.author  = 'buty4649'
  spec.summary = 'reddish'
  spec.bins    = ['reddish']

  spec.add_dependency 'mruby-process-pgrp'
  spec.add_dependency 'mruby-reddish-parser'
  spec.add_dependency 'mruby-kernel-ext'
  spec.add_dependency 'mruby-io'
  spec.add_dependency 'mruby-pack'
  spec.add_dependency 'mruby-print'
  spec.add_dependency 'mruby-mtest',   mgem: 'mruby-mtest'
  spec.add_dependency 'mruby-getopts', mgem: 'mruby-getopts'
end

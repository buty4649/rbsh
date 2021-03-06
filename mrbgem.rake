MRuby::Gem::Specification.new('reddish') do |spec|
  spec.license = 'MIT'
  spec.author  = 'buty4649'
  spec.summary = 'reddish'
  spec.bins    = ['reddish']

  spec.add_dependency 'mruby-dir'
  spec.add_dependency 'mruby-dir-glob'
  spec.add_dependency 'mruby-env'
  spec.add_dependency 'mruby-io'
  spec.add_dependency 'mruby-kernel-ext'
  spec.add_dependency 'mruby-onig-regexp'
  spec.add_dependency 'mruby-pack'
  spec.add_dependency 'mruby-print'
  spec.add_dependency 'mruby-require'
  spec.add_dependency 'mruby-process', mgem: 'mruby-process2'

  spec.add_dependency 'mruby-ruby-exec',      path: '../mrbgems/mruby-ruby-exec'
  spec.add_dependency 'mruby-reddish-parser', path: '../mrbgems/mruby-reddish-parser'
  spec.add_dependency 'mruby-signal-handler', path: '../mrbgems/mruby-signal-handler'

  spec.add_dependency 'mruby-exec',      github: 'haconiwa/mruby-exec'
  spec.add_dependency 'mruby-getopts',   github: 'buty4649/mruby-getopts', branch: 'add-prog-name'
  spec.add_dependency 'mruby-linenoise', github: 'buty4649/mruby-linenoise', branch: 'raise-ctrl-c'
  spec.add_dependency 'mruby-io-dup2',   github: 'buty4649/mruby-io-dup2', branch: 'main'
  spec.add_dependency 'mruby-io-fcntl',  github: 'buty4649/mruby-io-fcntl', branch: 'main'
  spec.add_dependency 'mruby-pp',        github: 'kou/mruby-pp'
  #spec.add_dependency 'mruby-process-pgrp', github: 'buty4649/mruby-process-pgrp', branch: 'main'
  spec.add_dependency 'mruby-process-pgrp', path: '/home/ykky/src/github.com/buty4649/mruby-process-pgrp'
end

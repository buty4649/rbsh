MRuby::Gem::Specification.new('mruby-bin-fdtest') do |spec|
  spec.license = 'MIT'
  spec.author  = 'buty4649'
  spec.summary = 'file descriptor test tools for reddish'
  spec.bins = %w(fdtest)

  spec.add_dependency 'mruby-dir-glob'
  spec.add_dependency 'mruby-iijson'
  spec.add_dependency 'mruby-io'
  spec.add_dependency 'mruby-onig-regexp'
  spec.add_dependency 'mruby-process'
end

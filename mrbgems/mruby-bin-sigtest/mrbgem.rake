MRuby::Gem::Specification.new('mruby-bin-fdtest') do |spec|
  spec.license = 'MIT'
  spec.author  = 'buty4649'
  spec.summary = 'signal test tools for reddish'
  spec.bins = %w(sigtest)

  spec.add_dependency 'mruby-io'
  spec.add_dependency 'mruby-signal-thread'
  spec.add_dependency 'mruby-process'
end

MRuby::Gem::Specification.new('reddish') do |spec|
  spec.license = 'MIT'
  spec.author  = 'buty4649'
  spec.summary = 'reddish'
  spec.bins    = ['reddish']

  spec.add_dependency 'mruby-print', :core => 'mruby-print'
  spec.add_dependency 'mruby-mtest', :mgem => 'mruby-mtest'
end

MRuby::Gem::Specification.new('mruby-signal-trap') do |spec|
  spec.license = 'MIT'
  spec.author  = 'buty4649'
  spec.summary = 'signal trap'

  spec.linker.flags << '-pthread'
end

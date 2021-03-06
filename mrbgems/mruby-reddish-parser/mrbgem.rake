MRuby::Gem::Specification.new('mruby-reddish-parser') do |spec|
  spec.license = 'MIT'
  spec.author  = 'buty4649'
  spec.summary = 'reddish-shell parser library'

  spec.add_dependency 'mruby-struct'
  spec.add_dependency 'mruby-array-ext'

  dir = spec.dir
  build_dir = spec.build_dir

  file "#{build_dir}/core/parser.tab.c" => ["#{dir}/core/parser.y"] do |t|
    yacc.run t.name, t.prerequisites.first
  end

  spec.objs << "#{build_dir}/core/parser.tab.o"
end

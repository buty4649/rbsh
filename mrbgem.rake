MRuby::Gem::Specification.new('reddish') do |spec|
  spec.license = 'MIT'
  spec.author  = 'MRuby Developer'
  spec.summary = 'reddish'
  spec.bins    = ['reddish']

  spec.add_dependency 'mruby-print', :core => 'mruby-print'
  spec.add_dependency 'mruby-mtest', :mgem => 'mruby-mtest'
  spec.add_dependency 'mruby-struct'

  dir = spec.dir
  build_dir = spec.build_dir

  file "#{dir}/src/parser.tab.c" => ["#{dir}/core/parser.y"] do |t|
    FileUtils.mkdir_p "#{dir}/src"
    yacc.run t.name, t.prerequisites.first
  end

  spec.objs << "#{build_dir}/src/parser.tab.o"
end

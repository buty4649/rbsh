module Reddish
  class Cli
    def self.start(args)
      opts = Shell.getopts(args)

      if opts["version"]
        show_version
      end

      Shell.new(opts).run
    end

    def self.show_version
      puts "#{$0}: #{Reddish::VERSION}"
      exit
    end
  end
end

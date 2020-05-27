module Reddish
  class Cli
    def self.start(args)
      Shell.new(args).run
    end
  end
end

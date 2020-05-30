module Reddish
  class Shell
    def initialize(args)
      @args = args
    end

    def run
      SignalThread.trap(:INT) do
        puts "^C"
      end

      if ENV["REDDISH_DEBUG"]
        ReddishParser.debug = true
      end

      while(line = linenoise("reddish> "))
        unless line.empty?
          cmdline = ReddishParser.parse(line)

          if cmdline
            cmdline.exec
          end
        end
      end
    end
  end
end

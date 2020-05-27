module Reddish
  class Shell
    def initialize(args)
      @args = args
    end

    def run
      SignalThread.trap(:INT) do
        puts "^C"
      end

      if debug = ENV["REDDISH_DEBUG"]
        Parser.debug = debug.to_i
      end

      while(line = linenoise("reddish> "))
        unless line.empty?
          cmdline = Reddish::Commandline.parse(line)

          if cmdline
            cmdline.exec
          end
        end
      end
    end
  end
end

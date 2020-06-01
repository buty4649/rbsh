module Reddish
  class Shell
    def initialize(args)
      @args = args
      @job  = JobControl.new
    end

    def run
      SignalThread.trap(:INT) do
        puts "^C"
      end

      if ENV["REDDISH_DEBUG"]
        ReddishParser.debug = true
      end

      while
        begin
          line = linenoise("reddish> ")
        rescue Errno::ENOTTY => e
          # bugs:
          # Errono::NOTTY occurs unintentionally.
          # (e.g. `echo hoge | reddish` )
        end
        break if line.nil?

        unless line.empty?
          parse_result = ReddishParser.parse(line)

          if parse_result
            @job.run(parse_result)
          end
        end
      end
    end
  end
end

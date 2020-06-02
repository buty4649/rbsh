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

      if ENV["REDDISH_PARSER_DEBUG"]
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
          begin
            parse_result = ReddishParser.parse(line)

            if parse_result
              @job.run(parse_result)
            end
          rescue => e
            puts "#{e.class} #{e.message}"
            if ENV['REDDISH_DEBUG']
              puts
              puts "backtrace:"
              e.backtrace.each_with_index do |t, i|
                puts " [#{i}] #{t}"
              end
            end
          end
        end
      end
    end
  end
end

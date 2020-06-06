module Reddish
  class Shell
    def initialize(args)
      class << args; include Getopts; end
      @opts = args.getopts("c:")
      @job  = JobControl.new
    end

    def read_from_tty
      linenoise("reddish> ")
    rescue Errno::ENOTTY => e
      # bugs:
      # Errono::NOTTY occurs unintentionally.
      # (e.g. `echo hoge | reddish` )
    end

    def run
      if ENV["REDDISH_PARSER_DEBUG"]
        ReddishParser.debug = true
      end

      if cmd = @opts["c"]
        parse_and_exec(cmd)
      else
        while line = read_from_tty
          parse_and_exec(line)
        end
      end
    end

    def parse_and_exec(line)
      return if line.nil? || line.empty?

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

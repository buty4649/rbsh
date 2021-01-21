module Reddish
  class Shell
    PS1 = "reddish> "

    def initialize(opts)
      @opts = opts
      @job = JobControl.new
      @executor = Executor.new
      @data_home = File.join(File.expand_path(XDG["CONFIG_HOME"]), "reddish")
    end

    def self.getopts(args)
      class << args; include Getopts; end
      opts = args.getopts("ic:", "version")
      if opts["?"]
        # Invalid option
        exit(2)
      end
      opts
    end

    def read_from_tty
      if @opts["i"]
        STDOUT.write(PS1)
        STDIN.gets
      else
        begin
          linenoise(PS1)
        rescue Errno::ENOTTY => e
          # bugs:
          # Errono::NOTTY occurs unintentionally.
          # (e.g. `echo hoge | reddish` )
        end
      end
    end

    def run
      if ENV["REDDISH_PARSER_DEBUG"]
        ReddishParser.debug = true
      end

      if File.exists?(history_file_path)
        Linenoise::History.load(history_file_path)
      elsif Dir.exists?(@data_home).!
        Dir.mkdir(@data_home)
      end

      BuiltinCommands.define_commands(@executor)

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
          @job.run(@executor, parse_result)

          if $?.success?
            Linenoise::History.add(line)
            Linenoise::History.save(history_file_path)
          end
        end
      rescue => e
        STDERR.puts "#{e.class} #{e.message}"
        if ENV['REDDISH_DEBUG']
          STDERR.puts
          STDERR.puts "backtrace:"
          e.backtrace.each_with_index do |t, i|
            STDERR.puts " [#{i}] #{t}"
          end
        end
      end
    end

    def history_file_path
      File.join(@data_home, "history.txt")
    end
  end
end

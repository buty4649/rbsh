module Reddish
  class Shell
    PS1 = "reddish> "
    PS2 = "reddish* "

    def initialize(opts)
      @opts = opts
      @job = JobControl.new
      @variable = Variable.new
      @data_home = File.join(File.expand_path(XDG["CONFIG_HOME"]), "reddish")
    end

    def self.getopts(args)
      class << args; include Getopts; end
      opts = args.getopts("ic:r", "version", "ruby")
      if opts["?"]
        # Invalid option
        exit(2)
      end
      args.optind.times { args.shift }
      opts["script"] = args.shift
      opts["args"] = args
      opts
    end

    def run
      if @opts["r"] || @opts["ruby"]
        script = @opts["script"] || "-"
        exit(Ruby.exec_from_file(script, *@opts["args"]))
      end

      if @variable["REDDISH_PARSER_DEBUG"]
        ReddishParser.debug = true
      end

      if @variable["REDDISH_LEXER_DEBUG"]
        ReddishParser.lexer_debug = true
      end

      unless Dir.exists?(@data_home)
        Dir.mkdir(@data_home)
      end

      r = reader
      cmdline = []
      need_next_list = false

      loop do
        line = r.readline(need_next_list ? PS2 : PS1)
        break if line.nil? && cmdline.empty?
        if line
          cmdline << line
          if line[-1] == "\\"
            need_next_list = true
            next
          end
        end
        parse_and_exec(cmdline.join("\n"))
        r.add_history(cmdline)
        need_next_list = false
      rescue ReddishParser::UnterminatedString, ReddishParser::UnexpectedKeyword => e
        if line.nil?
          need_next_list = false
          STDERR.puts "Unterminated string."
        else
          need_next_list = true
        end
      rescue Errno::EAGAIN, Errno::EWOULDBLOCK => e
        # reset command line
        need_next_list = false
      rescue => e
        need_next_list = false
        STDERR.puts "#{e.class} #{e.message}"
        if ENV['REDDISH_DEBUG']
          STDERR.puts
          STDERR.puts "backtrace:"
          e.backtrace.each_with_index do |t, i|
            STDERR.puts " [#{i}] #{t}"
          end
        end
      ensure
        cmdline = [] unless need_next_list
      end
    end

    def parse_and_exec(line)
      parse_result = ReddishParser.parse(line)

      if parse_result
        executor = Executor.new(@variable)
        BuiltinCommands.define_commands(executor)
        @job.run(executor, parse_result)
      end
    end

    def reader
      if cmd = @opts["c"]
        Reader::SingleCommand.new(cmd)
      elsif script = @opts["script"]
        file = File.open(script)
        Reader::File.new(file, false)
      elsif @opts["i"]
        Reader::File.new(STDIN, true)
      elsif STDIN.tty?
        Reader::Linenoise.new(@data_home)
      else
        # read from pipe
        Reader::File.new(STDIN, false)
      end
    end
  end
end

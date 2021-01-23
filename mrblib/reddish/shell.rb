module Reddish
  class Shell
    PS1 = "reddish> "
    PS2 = "reddish* "

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

    def readline(prompt)
      if @opts["i"]
        STDOUT.write(prompt)
        STDIN.gets
      else
        linenoise(prompt)
      end
    end

    def run
      if ENV["REDDISH_PARSER_DEBUG"]
        ReddishParser.debug = true
      end

      if ENV["REDDISH_LEXER_DEBUG"]
        ReddishParser.lexer_debug = true
      end

      Linenoise.multi_line = true
      if File.exists?(history_file_path)
        Linenoise::History.load(history_file_path)
      elsif Dir.exists?(@data_home).!
        Dir.mkdir(@data_home)
      end

      BuiltinCommands.define_commands(@executor)

      if cmd = @opts["c"]
        parse_and_exec(cmd)
      else
        cmdline = []
        need_next_list = false
        loop do
          line = readline(need_next_list ? PS2 : PS1)
          break if line.nil? && cmdline.empty?
          cmdline << line
          if line[-1] == "\\"
            need_next_list = true
            next
          end
          parse_and_exec(cmdline.join("\n"))

          cmdline.each do |cmd|
            Linenoise::History.add(cmd)
          end
          Linenoise::History.save(history_file_path)

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
    end

    def parse_and_exec(line)
      parse_result = ReddishParser.parse(line)

      if parse_result
        @job.run(@executor, parse_result)
      end
    end

    def history_file_path
      File.join(@data_home, "history.txt")
    end
  end
end

module Reddish
  class Executor

    def initialize(variable)
      @defined_command = {}
      @pgid = nil
      @loop_level = 0
      @breaking = 0
      @continuing = 0
      @variable = variable
    end

    def define_command(name, code)
      @defined_command[name.to_sym] = code
    end

    def reset
      @pgid = nil
    end

    def exec(command, opts={})
      return if @breaking.nonzero? || @continuing.nonzero?
      klass = command.class
      if klass == ReddishParser::Element::Command
        $? = command_exec(command, opts)
      elsif klass == ReddishParser::Element::Connector
        connector_exec(command, opts)
      elsif klass == ReddishParser::Element::IfStatement
        if command.async || opts[:async]
          pgid = @pgid || 0
          pid ||= Process.fork do
            Process.setpgid(0, pgid)
            if_statement(command, opts)
          end
          @pgid ||= pid
          if command.async || opts[:async]
            exit_status  = Process::Status.new(pid, nil)
          else
            _, exit_status = JobControl.start_sigint_trap(@pgid) { Process.wait2(pid) }
            reset
          end
          exit_status
        else
          if_statement(command, opts)
        end
      elsif klass == ReddishParser::Element::WhileStatement
        if command.async || opts[:async]
          pgid = @pgid || 0
          pid ||= Process.fork do
            Process.setpgid(0, pgid)
            while_statement(command, opts)
          end
          @pgid ||= pid
          if command.async || opts[:async]
            exit_status  = Process::Status.new(pid, nil)
          else
            _, exit_status = JobControl.start_sigint_trap(@pgid) { Process.wait2(pid) }
            reset
          end
          exit_status
        else
          while_statement(command, opts)
        end
      end
    end

    def command_exec(command, opts)
      env = command.env.map do |key, values|
        value = values.map{|v| word2str(v)}.flatten.join
        [key, value]
      end.to_h
      cmd, args = split_cmdline(command.cmdline)

      rc = setup_redirect_control(command.redirect)
      if fd = opts[:stdout]
        rc.append(:copywrite, 1, fd)
        rc.append(:close, fd)
      end
      if fd = opts[:stdin]
        rc.append(:copyread, 0, fd)
        rc.append(:close, fd)
      end

      if cmd.nil?
        rc.apply(true)
        env&.each do |name, value|
          if value.empty?
            ENV.delete(name)
          else
            ENV[name] = value
          end
        end
      elsif cmd =~ /^(break|continue|next)$/
        return rc.apply(true) { do_break_or_continue(cmd, args) }
      elsif cmd == "read"
        if command.async || opts[:async]
          pgid = @pgid || 0
          pid = Process.fork do
            Process.setpgid(0, pgid)
            rc.apply(true) { do_builtin_read(args) }
          end
          @pgid ||= pid
          return Process::Status.new(pid, nil)
        else
          return rc.apply(true) { do_builtin_read(args) }
        end
      elsif c = @defined_command[cmd.to_sym]
        old_env = env&.map do |name, value|
          old_value = ENV[name]
          ENV[name] = value
          [name, old_value]
        end

        # call defined command
        exit_status = rc.apply(true) { c.call(*args) }

        # restore env
        old_env.each {|n, v| ENV[n] = v } if old_env

        exit_status
      else
        progname = cmd
        assume_command = search_command(cmd)

        pgid = @pgid || 0
        pid = Process.fork do
          Process.setpgid(0, pgid)
          rc.clexec = false
          e = ENV.to_hash.merge(env || {})
          begin
            rc.apply
            Exec.execve_override_procname(e, progname, assume_command, *args)
          rescue Errno::ENOENT
            STDERR.puts "reddish-shell: Command '#{progname}' not found."
            Process.exit(127)
          rescue => e
            STDERR.puts "reddish-shell: #{e.message}"
            Process.exit(1)
          end
        end
        @pgid ||= pid

        if command.async || opts[:async]
          exit_status  = Process::Status.new(pid, nil)
        else
          _, exit_status = JobControl.start_sigint_trap(@pgid) { Process.wait2(pid) }
          reset
        end

        exit_status
      end
    end

    def connector_exec(connector, opts)
      if connector.type == :pipeline
        pipe_exec(connector, opts)
      else
        cmd1_opts = opts || {}
        cmd1_opts.merge({async: true}) if opts[:async]
        result = exec(connector.cmd1, cmd1_opts)

        if cmd2_exec?(connector.type, result)
          result = exec(connector.cmd2, opts)
        end

        result
      end
    end

    def pipe_exec(pipe, opts)
      r, w = IO.pipe
      exec(pipe.cmd1, opts.merge({stdout: w.fileno, async: true}))
      exec(pipe.cmd2, opts.merge({stdin:  r.fileno, stdout: opts[:stdout], async: true}))
      begin r.close ; rescue Errno::EBADF => e; end
      begin w.close ; rescue Errno::EBADF => e; end

      return if opts[:async]

      # Array<Array<pid, Process::Status>>
      result = JobControl.start_sigint_trap(@pgid) { Process.waitall }

      # Process:Status of cmd2
      result.last.last
    end

    def search_command(command)
      return command if command.include?("/")

      result = ENV["PATH"].split(/:/)
                          .map {|dir| "#{dir}/#{command}" }
                          .find {|path| File.exists?(path) }
      return result || command
    end

    def if_statement(statement, opts)
      rc = setup_redirect_control(statement.redirect)
      if fd = opts[:stdout]
        rc.append(:copywrite, 1, fd)
        rc.append(:close, fd)
      end
      if fd = opts[:stdin]
        rc.append(:copyread, 0, fd)
        rc.append(:close, fd)
      end

      rc.apply(true) do
        exec(statement.condition)
        state = statement.reverse ? $?.success?.! : $?.success?
        if state
          exec(statement.cmd1)
        elsif statement.cmd2
          exec(statement.cmd2)
        end

        # return last status
        $?
      end
    end

    def while_statement(statement, opts)
      rc = setup_redirect_control(statement.redirect)
      if fd = opts[:stdout]
        rc.append(:copywrite, 1, fd)
        rc.append(:close, fd)
      end
      if fd = opts[:stdin]
        rc.append(:copyread, 0, fd)
        rc.append(:close, fd)
      end

      rc.apply(true) do
        @loop_level += 1
        loop do
          exec(statement.condition)
          state = statement.reverse ? $?.success?.! : $?.success?
          unless state
            @breaking -= 1 if @breaking.nonzero?
            @continuing -= 1 if @continuing.nonzero?
            break
          end

          exec(statement.cmd)

          if @breaking.nonzero?
            @breaking -= 1
            break
          end

          if @continuing.nonzero?
            @continuing -= 1
            break if @continuing.nonzero?
          end
        end

        @loop_level -= 1

        # return last status
        $?
      end
    end

    def cmd2_exec?(t, r)
      (t == :and && r.success?)   ||
      (t == :or  && r.success?.!) ||
      t == :semicolon ||
      t == :async
    end

    def split_cmdline(cmdline)
      return if cmdline.nil?

      list = cmdline.map do |c|
        next if c.empty?
        d = c.map{|d| word2str(d)}.flatten.compact
        d.empty? ? nil : d.join
      end.compact
      [list.shift, list]
    end

    def do_break_or_continue(cmd, args)
      if @loop_level.zero?
        STDERR.puts %|reddish-shell: #{cmd}: only meaningful in a `for', `while', or `until' loop|
        return Process::Status.new(nil, 1)
      end

      level = 1

      unless args.empty?
        if args.length > 1
          STDERR.puts %|reddish-shell: #{cmd}: too many arguments|
          return Process::Status.new(nil, 1)
        end

        if args.first !~ /^\d+/
          STDERR.puts %|reddish-shell: #{cmd}: numeric argument require|
          return Process::Status.new(nil, 1)
        end

        level = args.first.to_i
      end

      if level <= 0
        STDERR.puts %|reddish-shell: #{cmd}: loop count out of range|
        return Process::Status.new(nil, 1)
      end

      level = @loop_level if level >= @loop_level

      if cmd =~ /continue|next/
        @continuing = level
      elsif cmd == "break"
        @breaking = level
      end
    end

    def do_builtin_read(args)
      if args.select{|a| a.length > 0 && a !~ /\A\w+$/ }.size.nonzero?
        STDERR.puts %|reddish-shell: #{read}: not a valid identifier|
        return Process::Status.new(nil, 1)
      end

      ifs = @variable["IFS"] || /[ \t\n]/
      begin
        line = STDIN.readline.chomp!&.split(ifs, args.length)
      rescue EOFError => e
        # suppress error
      end

      return Process::Status.new(nil, 1) if line.nil?

      args.each do |name|
        @variable[name] = line.shift
      end

      Process::Status.new(nil, 0)
    end

    def word2str(word)
      return "" if word.nil? || word.type == :separator

      str = word.to_s
      return str if word.type == :quote

      str = str.gsub(/(?<!\\)\${(\w+)}/) { @variable[$1] || "" }
                .gsub(/(?<!\\)\$(\w+)/)   { @variable[$1] || "" }
                .gsub(/(?<!\\)\$\?/)      { $?.nil? ? 0 : $? >> 8 }
                .gsub(/\\\$/, "$")

      return str if word.type != :execute

      parse = ReddishParser.parse(str)

      r, w = IO.pipe
      Executor.new(@variable).exec(parse, {stdout: w.fileno})
      w.close
      result = r.read.chomp.split(/ \t\n/)
      r.close

      result.nil? ? nil : result
    end

    def setup_redirect_control(redirect)
      rc = RedirectControl.new

      redirect&.each do |r|
        filename = word2str(r.filename)
        rc.append(r.type, r.dest, r.src, filename)
      end

      rc
    end
  end
end

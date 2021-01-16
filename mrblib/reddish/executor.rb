module Reddish
  class Executor

    class << self
      def word2str(word)
        return "" if word.nil? || word.type == :separator

        str = word.to_s
        return str if word.type == :quote

        str.gsub(/\${(\w+)}/) { ENV[$1] || "" }
           .gsub(/\$(\w+)/)   { ENV[$1] || "" }
           .gsub(/\$\?/) { $?.nil? ? 0 : $? >> 8 }
      end
    end

    def initialize
      @defined_command = {}
      @pgid = nil
    end

    def define_command(name, code)
      @defined_command[name.to_sym] = code
    end

    def reset
      @pgid = nil
    end

    def exec(command, opts={})
      klass = command.class
      if klass == ReddishParser::Element::Command
        command_exec(command, opts)
      elsif klass == ReddishParser::Element::Connector
        connector_exec(command, opts)
      elsif klass == ReddishParser::Element::Pipeline
        pipe_exec(command, opts)
      elsif klass == ReddishParser::Element::IfStatement
        if command.async
          Process.fork do
            Process.setpgid(0, 0)
            @pgid = $$
            if_statement(command)
          end
        else
          if_statement(command)
        end
      end
    end

    def command_exec(command, opts)
      redirect = command.redirect || []
      if fd = opts[:stdout]
        redirect.unshift(ReddishParser::Element::Redirect.new(:close, fd))
        redirect.unshift(ReddishParser::Element::Redirect.new(:copywrite, 1, fd))
      end
      if fd = opts[:stdin]
        redirect.unshift(ReddishParser::Element::Redirect.new(:close, fd))
        redirect.unshift(ReddishParser::Element::Redirect.new(:copyread, 0, fd))
      end
      rc = RedirectControl.new(redirect)

      env = command.env.map{|k,v| [k, Executor.word2str(v)]}.to_h
      cmd, args = split_cmdline(command.cmdline)

      if cmd.nil?
        rc.apply(true)
        env&.each do |name, value|
          if value.empty?
            ENV.delete(name)
          else
            ENV[name] = value
          end
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
      cmd1_opts = opts || {}
      cmd1_opts.merge({async: true}) if opts[:async]
      result = exec(connector.cmd1, cmd1_opts)

      if cmd2_exec?(connector.type, result)
        result = exec(connector.cmd2, opts)
      end

      result
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

    def if_statement(statement)
      RedirectControl.new(statement.redirect).apply(true) do
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

    private
    def cmd2_exec?(t, r)
      (t == :and && r.success?)   ||
      (t == :or  && r.success?.!) ||
      t == :semicolon ||
      t == :async
    end

    def split_cmdline(cmdline)
      return if cmdline.nil?

      list = cmdline.map{|c| c.map{|d| Executor.word2str(d)}.join }
      list.select!{|l| l.empty?.!}
      [list.first, list[1..-1]]
    end
  end
end

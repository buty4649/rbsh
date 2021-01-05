module Reddish
  class Executor
    def initialize
      @cmd = {}
      @pgid = nil
    end

    def define_command(name, code)
      @cmd[name.to_sym] = code
    end

    def reset
      @pgid = nil
    end

    def exec(command, opts={})
      klass = command.class
      if klass == Element::Command
        command_exec(command, opts)
      elsif klass == Element::Connector
        connector_exec(command, opts)
      elsif klass == Element::Pipe
        pipe_exec(command, opts)
      end
    end

    def command_exec(command, opts)
      env, cmd, args = split(command.wordlist)
      return if env.nil? && cmd.nil?

      redirect = command.redirect || []
      if fd = opts[:stdout]
        redirect.unshift(Element::Redirect.new(:close, fd))
        redirect.unshift(Element::Redirect.new(:copywrite, 1, fd))
      end
      if fd = opts[:stdin]
        redirect.unshift(Element::Redirect.new(:close, fd))
        redirect.unshift(Element::Redirect.new(:copyread, 0, fd))
      end
      rc = RedirectControl.new(redirect)

      if env && cmd.nil?
        rc.apply(true) do
          name, value = env.split("=", 2)
          ENV[name] = value
          Process::Status.new($$, 0)
        end
      elsif c = @cmd[cmd.to_sym]
        old_env = env&.map do |e|
          name, value = e.split("=", 2)
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
        local_env = ENV.to_hash.merge(env || {})
        progname = cmd
        assume_command = search_command(cmd)

        pgid = @pgid || 0
        pid = Process.fork do
          Process.setpgid(0, pgid)
          rc.clexec = false
          rc.apply
          begin
            Exec.execve_override_procname(local_env, progname, assume_command, *args)
          rescue Errno::ENOENT
            STDERR.puts "reddish-shell: Command '#{progname}' not found."
            Process.exit(127)
          end
        end
        @pgid ||= pid

        if command.async || opts[:async]
          exit_status  = Process::Status.new(pid, nil)
        else
          _, exit_status = JobControl.start_sigint_trap(@pgid) { Process.wait2(pid) }
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

    def split(wordlist)
      env, cmd = nil
      args = []
      wordlist.to_a.each do |word|
        if cmd.nil?
          if word.index("=")
            env ||= []
            env << word
          else
            cmd = word
          end
        else
          args << word
        end
      end
      [env, cmd, args]
    end

    def search_command(command)
      return command if command.include?("/")

      result = ENV["PATH"].split(/:/)
                           .map {|dir| "#{dir}/#{command}" }
                           .find {|path| File.exists?(path) }
      return result || command
    end

    private
    def cmd2_exec?(t, r)
      (t == :and && r.success?)   ||
      (t == :or  && r.success?.!) ||
      t == :semicolon ||
      t == :async
    end
  end
end

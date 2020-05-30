module ReddishParser
  class Command
    attr_accessor :redirect

    def initialize(wordlist)
      @cmd = wordlist.shift
      @args = wordlist
      @redirect = []
    end

    def exec
      pid = Process.fork {
        command = assume_command
        args = @args.to_a.map(&:to_s)

        @redirect.each(&:apply)

        Exec.execve_override_procname(ENV.to_hash, @cmd.to_s, command, *args)
      }
      _, st = Process.wait2(pid)
      st
    end

    def assume_command
      command = @cmd.to_s
      return command if command.include?("/")

      result = ENV["PATH"].split(/:/)
                          .map {|dir| "#{dir}/#{command}" }
                          .find {|path| File.exists?(path) }
      return result || command
    end
  end
end

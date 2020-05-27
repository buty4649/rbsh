module Reddish
  class Command

    def initialize(wordlist)
      @cmd = wordlist.shift
      @args = wordlist
    end

    def exec
      pid = Process.fork {
        Process.exec(assume_command, @args.to_a.map(&:to_s))
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

module ReddishParser
  module Element
    class Command
      def initialize(wordlist, redirect=nil)
        @wordlist = wordlist
        @redirect = redirect
        @async = false
      end

      def redirect=(redirect_list)
        @redirect = redirect_list
      end

      def async=(flag)
        @async = flag
      end

      def exec
        pid = Process.fork {
          command = @wordlist.first.to_s
          next if command.empty?
          assume_command = Utils.search_command(command)
          args = @wordlist.to_a
          progname = args.shift

          if @redirect
            @redirect.each(&:apply)
          end

          Exec.execve_override_procname(ENV.to_hash, progname, assume_command, *args)
        }

        if @async
          st = Process::Status.new(pid, nil)
        else
          _, st = Process.wait2(pid)
        end

        st
      end
    end
  end
end

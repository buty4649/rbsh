module ReddishParser
  module Element
    class Command
      def initialize(wordlist)
        @wordlist = wordlist
        @redirect = nil
      end

      def redirect=(redirect_list)
        @redirect = redirect_list
      end

      def exec(fg=true)
        pid = Process.fork {
          command = Utils.search_command(@wordlist.first.to_s)
          args = @wordlist.to_a
          progname = args.shift

          if @redirect
            @redirect.each(&:apply)
          end

          Exec.execve_override_procname(ENV.to_hash, progname, command, *args)
        }

        if fg
          _, st = Process.wait2(pid)
        else
          st = Process::Status.new(pid, nil)
        end

        st
      end
    end
  end
end

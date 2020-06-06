module ReddishParser
  module Element
    class Command
      attr_accessor :redirect

      def initialize(wordlist, redirect=nil)
        @wordlist = wordlist
        @redirect = redirect || []
        @async = false
      end

      def add_redirect(redirect)
        if redirect.class == Array
          @redirect += redirect
        else
          @redirect <<= redirect
        end
      end

      def async=(flag)
        @async = flag
      end

      def exec(opts={})
        pid = Process.fork {
          command = @wordlist.first.to_s
          next if command.empty?
          assume_command = Utils.search_command(command)
          args = @wordlist.to_a
          progname = args.shift

          if fd = opts[:stdout]
            Element::Redirect.new(RedirectType::COPYWRITE, 1, fd).apply
            Element::Redirect.new(RedirectType::CLOSE, fd).apply
          end

          if fd = opts[:stdin]
            Element::Redirect.new(RedirectType::COPYREAD, 0, fd).apply
            Element::Redirect.new(RedirectType::CLOSE, fd).apply
          end

          if @redirect
            @redirect.each(&:apply)
          end

          Exec.execve_override_procname(ENV.to_hash, progname, assume_command, *args)
        }

        if @async || opts[:async]
          st = Process::Status.new(pid, nil)
        else
          _, st = Action.start_sigint_trap([pid]) { Process.wait2(pid) }
        end

        st
      end
    end
  end
end

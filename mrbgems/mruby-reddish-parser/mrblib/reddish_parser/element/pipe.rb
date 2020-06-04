module ReddishParser
  module Element
    class Pipe
      attr_accessor :cmd1, :cmd2

      def initialize(cmd1, cmd2)
        @cmd1 = cmd1
        @cmd2 = cmd2
        @async= false
      end

      def async=(flag)
        @async = flag
      end

      def exec(opts={})
        r, w = IO.pipe
        st1 = @cmd1.exec({stdout: w.fileno, async: true})
        st2 = @cmd2.exec({stdin:  r.fileno, stdout: opts[:stdout], async: true})

        r.close
        w.close

        # Array<Array<pid, Process::Status>>
        result = Process.waitall

        # Process:Status of cmd2
        result.last.last
      end
    end
  end
end

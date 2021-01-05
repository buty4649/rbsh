module Reddish
  module BuiltinCommands
    module Base
      def success
        $? = Process::Status.new($$, 0)
      end

      def error(cmd, msg=nil, status=1)
        STDERR.puts "reddish: #{cmd}: #{msg}" if msg
        $? = Process::Status.new($$, status)
      end

      def getopts(progname, argv, shortopt, *longopts)
        old_progname = $0
        $0 = $PROGRAM_NAME = progname

        class << argv; include Getopts; end
        opts = argv.getopts(shortopt, *longopts)

        $0 = $PROGRAM_NAME = old_progname

        # ? is invalid option
        opts["?"] ? nil : [opts, argv.optind]
      end
    end
  end
end

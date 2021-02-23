module ReddishParser
  module Element
    class ForStatement
      attr_reader :varname, :list, :cmd, :redirect
      attr_accessor :async

      def initialize(varname, list, cmd)
        @varname = varname
        @list = list
        @cmd = cmd
        @async = false
      end

      def append_redirect(redirect)
        @redirect ||= []
        @redirect += redirect
      end
    end
  end
end

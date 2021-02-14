module ReddishParser
  module Element
    class WhileStatement
      attr_reader :condition, :cmd, :redirect
      attr_accessor :async

      def initialize(condition, cmd=nil)
        @condition = condition
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

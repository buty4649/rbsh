module ReddishParser
  module Element
    class WhileStatement
      attr_reader :condition, :reverse, :cmd, :redirect
      attr_accessor :async

      def initialize(condition, reverse, cmd=nil)
        @condition = condition
        @reverse = reverse
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

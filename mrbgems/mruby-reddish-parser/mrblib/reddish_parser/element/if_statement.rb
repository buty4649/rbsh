module ReddishParser
  module Element
    class IfStatement
      attr_reader :condition, :reverse, :cmd1, :cmd2
      attr_accessor :async

      def initialize(condition, reverse, cmd1, cmd2=nil)
        @condition = condition
        @reverse = reverse
        @cmd1 = cmd1
        @cmd2 = cmd2
        @async = false
      end
    end
  end
end

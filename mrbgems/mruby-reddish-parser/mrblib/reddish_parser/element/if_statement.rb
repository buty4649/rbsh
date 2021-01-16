module ReddishParser
  module Element
    class IfStatement
      attr_reader :condition, :cmd1, :cmd2
      attr_accessor :async

      def initialize(type, condition, cmd1, cmd2=nil)
        @condition = condition
        @cmd1 = cmd1
        @cmd2 = cmd2
        @async = false
      end
    end
  end
end

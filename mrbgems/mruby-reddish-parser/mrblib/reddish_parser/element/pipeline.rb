module ReddishParser
  module Element
    class Pipeline
      attr_reader :cmd1, :cmd2

      def initialize(cmd1, cmd2)
        @cmd1 = cmd1
        @cmd2 = cmd2
      end
    end
  end
end

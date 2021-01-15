module ReddishParser
  module Element
    class Connector
      attr_reader :type, :cmd1, :cmd2

      def initialize(type, cmd1, cmd2)
        @type = type
        @cmd1 = cmd1
        @cmd2 = cmd2
      end
    end
  end
end

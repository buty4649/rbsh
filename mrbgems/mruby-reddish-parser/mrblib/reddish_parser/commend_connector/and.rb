module ReddishParser
  module CommandConnector
    class And
      def initialize(cmd1, cmd2)
        @cmd1 = cmd1
        @cmd2 = cmd2
      end

      def exec
        result = @cmd1.exec
        if result.success?
          result = @cmd2.exec
        end
      end
    end
  end
end

module Reddish
  module CommandConnector
    class Or
      def initialize(cmd1, cmd2)
        @cmd1 = cmd1
        @cmd2 = cmd2
      end

      def exec
        result = @cmd1.exec
        return result if result.success?

        @cmd2.exec
      end
    end
  end
end

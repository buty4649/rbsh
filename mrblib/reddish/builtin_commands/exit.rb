module Reddish
  module BuiltinCommands
    module Exit
      include Base

      def Exit(*args)
        Process.exit(0)
      end
    end
    extend Exit
  end
end

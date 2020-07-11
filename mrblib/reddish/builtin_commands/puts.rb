module Reddish
  module BuiltinCommands
    module Puts
      include Base

      def puts(*args)
        args.each {|arg| STDOUT.puts escape(arg) }
        success
      end
    end
    extend Puts
  end
end

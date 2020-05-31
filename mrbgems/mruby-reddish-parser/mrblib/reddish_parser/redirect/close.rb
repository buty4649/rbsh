module ReddishParser
  module Redirect
    class Close < Struct.new(:fd)
      def apply
        IO._sysclose(self[:fd])
        self[:fd]
      end
    end
  end
end

module ReddishParser
  module Redirect
    class Read < Base
      def initialize(filename, fd)
        @mode = "r"
        @perm = 0644
        super(filename, fd)
      end
    end
  end
end

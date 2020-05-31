module ReddishParser
  module Redirect
    class Write < Base
      def initialize(filename, fd)
        @mode = "w"
        @perm = 0644
        super(filename, fd)
      end
    end
  end
end

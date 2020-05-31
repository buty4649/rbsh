module ReddishParser
  module Redirect
    class ReadWrite < Base
      def initialize(filename, fd)
        @mode = "a+"
        @perm = 0644
        super(filename, fd)
      end
    end
  end
end

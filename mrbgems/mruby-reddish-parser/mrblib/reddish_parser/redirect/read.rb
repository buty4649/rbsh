module ReddishParser
  module Redirect
    class Read < Struct.new(:filename)

      def apply
        filename = self[:filename].to_s

        new_fd = IO.sysopen(filename, "r")
        IO.dup2(new_fd, STDIN.fileno)
        STDIN.close

        new_fd
      end

    end
  end
end

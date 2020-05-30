module ReddishParser
  module Redirect
    class Append < Struct.new(:filename)

      def apply
        filename = self[:filename].to_s

        new_fd = IO.sysopen(filename, "a", 0644)
        IO.dup2(new_fd, STDOUT.fileno)
        STDOUT.close

        new_fd
      end

    end
  end
end

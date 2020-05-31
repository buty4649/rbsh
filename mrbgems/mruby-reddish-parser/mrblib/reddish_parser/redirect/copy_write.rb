module ReddishParser
  module Redirect
    class CopyWrite < Struct.new(:src_fd, :dest_fd)
      def apply
        new_fd = IO.new(self[:dest_fd], "w", 0644)
        IO.dup2(self[:src_fd], new_fd.fileno)
        self[:dest_fd]
      end
    end
  end
end

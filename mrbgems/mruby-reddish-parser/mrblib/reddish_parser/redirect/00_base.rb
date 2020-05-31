# Required before append.rb.
module ReddishParser
  module Redirect
    class Base
      def initialize(filename, fd)
        @filename = filename.to_s
        @fd = fd.to_i
      end

      def mode
        @mode || "r"
      end

      def perm
        @perm || 0644
      end

      def apply
        new_fd = IO.sysopen(@filename, mode, perm)

        if new_fd != @fd
          IO.dup2(new_fd, @fd)
          IO.new(new_fd).close
          unless fd_opened?
            IO.new(@fd).close_on_exec = false
          end
        else
          IO.open(new_fd).close_on_exec = false
        end

        new_fd
      end

      def fd_opened?
        io = IO.new(@fd)
        io.stat
        return true
      rescue Errno::EBADF
        return false
      end
    end
  end
end

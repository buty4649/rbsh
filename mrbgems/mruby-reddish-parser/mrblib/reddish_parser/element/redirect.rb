module ReddishParser
  module Element
    class Redirect < Struct.new(:type, :dest_fd, :src_fd, :filename)
      def apply
        case self[:type]
        when RedirectType::APPEND     then open_and_dup("a", 0644)
        when RedirectType::CLOSE      then close
        when RedirectType::COPYREAD   then copy("r", 0644)
        when RedirectType::COPYWRITE  then copy("w", 0644)
        when RedirectType::READ       then open_and_dup("r", 0644)
        when RedirectType::READWRITE  then open_and_dup("a+", 0644)
        when RedirectType::WRITE      then open_and_dup("w", 0644)
        end
      end

      private
      def close
        IO._sysclose(self[:dest_fd])
        self[:dest_fd]
      end

      def copy(mode, perm)
        new_fd = IO.new(self[:dest_fd], mode, perm)
        IO.dup2(self[:src_fd], new_fd.fileno)
        new_fd.close_on_exec = false
        self[:dest_fd]
      end

      def open_and_dup(mode, perm)
        filename = self[:filename].to_s
        dest_fd  = self[:dest_fd]

        new_fd = IO.sysopen(filename, mode, perm)
        if new_fd != dest_fd
          IO.dup2(new_fd, dest_fd)
          IO.new(new_fd).close
          new_fd = dest_fd
        end
        IO.open(new_fd).close_on_exec = false

        new_fd
      end
    end
  end
end

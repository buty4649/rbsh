module Reddish
  class RedirectControl
    attr_accessor :clexec
    SHELL_FD_BASE = 10

    def initialize(redirect)
      @redirect = redirect || []
      @clexec = true
      @savefd = false
      @original_fd = {}
      @opened_fd = []
    end

    def apply(oneshot=false, &blk)
      @savefd = oneshot

      @redirect.each do |r|
        case r.type
        when :append     then open_and_dup(r.dest_fd, r.filename.to_s, "a")
        when :close      then close(r.dest_fd)
        when :copyread   then copy(r.dest_fd, r.src_fd, "r")
        when :copywrite  then copy(r.dest_fd, r.src_fd, "w")
        when :read       then open_and_dup(r.dest_fd, r.filename.to_s, "r")
        when :readwrite  then open_and_dup(r.dest_fd, r.filename.to_s, "a+")
        when :write      then open_and_dup(r.dest_fd, r.filename.to_s, "w")
        end
      end

      result = blk.call if blk
      restore if oneshot

      result
    end

    def restore
      @original_fd.each do |src_fd, dest_fd|
        close(src_fd)
        IO::fcntl(dest_fd, IO::F_DUPFD, src_fd)
      end

      (@opened_fd + @original_fd.values).each do |fd|
        close(fd)
      end
    end

    private
    def close(fd)
      IO._sysclose(fd)
    end

    def copy(dest_fd, src_fd, mode)
      if @savefd && dest_fd <= 2
        @original_fd[dest_fd] ||= IO::fcntl(dest_fd, IO::F_DUPFD, SHELL_FD_BASE)
      end

      IO.dup2(src_fd, dest_fd)
      new_fd = IO.new(dest_fd, mode)
      new_fd.close_on_exec = @clexec
    end

    def open_and_dup(dest_fd, filename, mode)
      new_fd = IO.sysopen(filename, mode)
      if new_fd != dest_fd
        copy(dest_fd, new_fd, mode)
        close(new_fd)
        new_fd = dest_fd
      end
      IO.open(new_fd).close_on_exec = @clexec

      if @savefd
        @opened_fd << new_fd if new_fd > 2
      end

      new_fd
    end
  end
end

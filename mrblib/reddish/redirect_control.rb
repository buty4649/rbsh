module Reddish
  class RedirectControl
    class FileNotFound < StandardError; end

    attr_accessor :clexec
    SHELL_FD_BASE = 10

    def initialize
      @redirect = []
      @clexec = true
      @savefd = false
      @original_fd = {}
      @opened_fd = []
    end

    def append(type, dest, src=nil, filename=nil)
      @redirect <<= [type, dest, src, filename]
    end

    def apply(oneshot=false, &blk)
      @savefd = oneshot

      @redirect.each do |type, dest, src, filename|
        case type
        when :append     then open_and_dup(dest, filename, "a")
        when :close      then close(dest)
        when :copyread   then copy(dest, src, "r")
        when :copywrite  then copy(dest, src, "w")
        when :read       then open_and_dup(dest, filename, "r")
        when :readwrite  then open_and_dup(dest, filename, "a+")
        when :write      then open_and_dup(dest, filename, "w")
        end
      end

      result = blk.call if blk
    rescue Errno::ENOENT => e
      raise FileNotFound.new(e.message)
    ensure
      restore if oneshot
    end

    def restore
      @original_fd.each do |src, dest|
        close(src)
        IO::fcntl(dest, IO::F_DUPFD, src)
      end

      (@opened_fd + @original_fd.values).each do |fd|
        close(fd)
      end
    end

    private
    def close(fd)
      IO._sysclose(fd)
    end

    def copy(dest, src, mode)
      if @savefd && dest <= 2
        @original_fd[dest] ||= IO::fcntl(dest, IO::F_DUPFD, SHELL_FD_BASE)
      end

      IO.dup2(src, dest)
      new_fd = IO.new(dest, mode)
      new_fd.close_on_exec = @clexec
    end

    def open_and_dup(dest, filename, mode)
      new_fd = IO.sysopen(filename, mode)
      if new_fd != dest
        copy(dest, new_fd, mode)
        close(new_fd)
        new_fd = dest
      end
      IO.open(new_fd).close_on_exec = @clexec

      if @savefd
        @opened_fd << new_fd if new_fd > 2
      end

      new_fd
    end
  end
end

module ReddishParser
  class Action
    class << self
      def make_word_list(word)
        WordList.new(word)
      end

      def add_to_word_list(dest, word)
        dest.add(word)
        dest
      end

      def make_command(wordlist)
        Command.new(wordlist)
      end

      def make_and_command_connector(cmd1, cmd2)
        CommandConnector::And.new(cmd1, cmd2)
      end

      def make_or_command_connector(cmd1, cmd2)
        CommandConnector::Or.new(cmd1, cmd2)
      end

      def assign_read_redirect(cmd, wordlist, fd=0)
        cmd.redirect << Redirect::Read.new(wordlist, fd.to_i)
        cmd
      end

      def assign_write_redirect(cmd, wordlist, fd=1)
        cmd.redirect << Redirect::Write.new(wordlist, fd.to_i)
        cmd
      end

      def assign_append_redirect(cmd, wordlist, fd=1)
        cmd.redirect << Redirect::Append.new(wordlist, fd.to_i)
        cmd
      end

      def assign_read_write_redirect(cmd, wordlist, fd=0)
        cmd.redirect << Redirect::ReadWrite.new(wordlist, fd.to_i)
        cmd
      end

      def assign_copy_read_redirect(cmd, src_fd, dest_fd=0)
        cmd.redirect << Redirect::CopyRead.new(src_fd.to_s.to_i, dest_fd.to_i)
        cmd
      end

      def assign_copy_write_redirect(cmd, src_fd, dest_fd=1)
        cmd.redirect << Redirect::CopyWrite.new(src_fd.to_i, dest_fd.to_i)
        cmd
      end

      def assign_close_redirect(cmd, fd)
        cmd.redirect << Redirect::Close.new(fd.to_i)
        cmd
      end
    end
  end
end

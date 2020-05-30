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

      def assign_read_redirect(cmd, wordlist)
        cmd.redirect << Redirect::Read.new(wordlist)
        cmd
      end

      def assign_write_redirect(cmd, wordlist)
        cmd.redirect << Redirect::Write.new(wordlist)
        cmd
      end

      def assign_append_redirect(cmd, wordlist)
        cmd.redirect << Redirect::Append.new(wordlist)
        cmd
      end
    end
  end
end

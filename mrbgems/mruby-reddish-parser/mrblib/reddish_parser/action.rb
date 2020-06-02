module ReddishParser
  class Action
    class << self
      def make_word_list(word)
        Element::WordList.new(word)
      end

      def add_to_word_list(dest, word)
        dest.add(word)
        dest
      end

      def make_command(wordlist, redirect_list=nil)
        Element::Command.new(wordlist, redirect_list)
      end

      def make_command_list(command, async=false)
        if command.class == Element::Connector
          command.cmd2.async = async
        else
          command.async = async
        end
        [command]
      end

      def make_async_command(command)
        [command]
      end

      def make_command_connector(type, cmd1, cmd2)
        t = ConnectorType.const_get(type.to_s)
        if t.nil?
          raise UnknwonType.new("unknwon connector type: #{type}")
        end
        Element::Connector.new(t, cmd1, cmd2)
      end

      def make_redirect(type, dest_fd, *args)
        t = RedirectType.const_get(type.to_s)
        if t.nil?
          raise UnknwonType.new("unknwon redirect type: #{type}")
        end
        redirect = Element::Redirect.new(type, dest_fd.to_i)
        while (arg = args.shift) do
          if arg.class == ReddishParser::Element::WordList
            redirect.filename = arg
          else
            redirect.src_fd = arg.to_i
          end
        end
        redirect
      end

      def make_redirect_list(redirect)
        redirect.class == Array ? redirect : [redirect]
      end

      def add_redirect_list(list, redirect)
        if redirect.class == Array
          list += redirect
        else
          list << redirect
        end
      end

      def assgin_redirect_list(cmd, redirect_list)
        cmd.redirect = redirect_list
        cmd
      end
    end
  end
end

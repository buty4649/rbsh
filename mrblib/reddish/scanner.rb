module Reddish
  class Scanner < ReddishParser::Action
    def on_word(data, before=nil)
      type, word = data
      em = Element::Word.new(type, word)
      if before
        before << em
      else
        Element::WordList.new(em)
      end
    end

    def on_redirect(type, dest, *args)
      redirect = Element::Redirect.new(type, dest.to_i)
      while arg = args.shift
        if arg.class == Element::WordList
          redirect.filename = arg
        else
          redirect.src_fd = arg.to_i
        end
      end
      redirect
    end

    def on_redirect_list(redirect, before=nil)
      if before
        if redirect.class == Array
          before += redirect
        else
          before << redirect
        end
      else
        redirect.class == Array ? redirect : [redirect]
      end
    end

    def on_command(data, redirect=nil)
      if data.class == Element::Command
        data.add_redirect(redirect)
      else
        Element::Command.new(data, redirect)
      end
    end

    def on_command_list(cmd, async=false)
      if async
        if cmd.class == Element::Connector
          cmd.cmd2.async = async
        else
          cmd.async = async
        end
      end
      [cmd]
    end

    def on_pipeline(cmd1, cmd2)
      Element::Pipe.new(cmd1, cmd2)
    end

    def on_connector(type, cmd1, cmd2)
      Element::Connector.new(type, cmd1, cmd2)
    end
  end
end

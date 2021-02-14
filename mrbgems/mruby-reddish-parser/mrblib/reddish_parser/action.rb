module ReddishParser
  class Action
    def on_word(word)
      Element::Word.new(word)
    end

    def on_redirect(type, dest, *args)
      while arg = args.shift
        if arg.class == Array
          filename = Element::Word.new(arg)
        else
          src = arg
        end
      end

      case type
      when :copyreadclose
        [Element::Redirect.new(:copyread, dest, src), Element::Redirect.new(:close, src)]
      when :copywriteclose
        [Element::Redirect.new(:copywrite, dest, src), Element::Redirect.new(:close, src)]
      when :copystdoutstderr
        [Element::Redirect.new(:write, 1, nil, filename), Element::Redirect.new(:copywrite, 2, 1)]
      else
        [Element::Redirect.new(type, dest, src, filename)]
      end
    end

    def on_append_redirect(command, redirect)
      command.append_redirect(redirect)
    end

    def on_async(cmd)
      target = cmd.class == Element::Connector ? cmd.cmd2 :  cmd
      target.async = true
      cmd
    end

    def on_command(elements)
      cmdline, redirect = nil
      elements.each do |element|
        if element.class == Element::Word
          cmdline ||= []
          cmdline << element
        elsif element.class == Array
          redirect ||= []
          redirect += element
        end
      end

      Element::Command.new(cmdline, redirect)
    end

    def on_pipeline(cmd1, cmd2, redirect)
      if redirect
        target = cmd1.class == Element::Connector ? cmd1.cmd2 : cmd2
        target.append_redirect(Element::Redirect.new(:copywrite, 2, 1, nil))
      end

      Element::Connector.new(:pipeline, cmd1, cmd2)
    end

    def on_connector(type, cmd1, cmd2)
      if type == :async
        cmd1.async = true
        Element::Connector.new(:semicolorn, cmd1, cmd2)
      else
        Element::Connector.new(type, cmd1, cmd2)
      end
    end

    def on_if_stmt(condition, reverse, cmd1=nil, cmd2=nil)
      if cmd1
        Element::IfStatement.new(condition, reverse, cmd1, cmd2)
      else
        if condition_cmd = dig_condition_command!(condition)
          Element::IfStatement.new(condition_cmd, reverse, condition, cmd2)
        else
          Element::IfStatement.new(condition, reverse, nil, cmd2)
        end
      end
    end

    def on_while_stmt(condition, cmd=nil)
      if cmd
        Element::WhileStatement.new(condition, cmd)
      else
        if condition_cmd = dig_condition_command!(condition)
          Element::WhileStatement.new(condition_cmd, condition)
        else
          Element::WhileStatement.new(condition, nil)
        end
      end
    end

    def on_error(msg, state)
      if state.first.zero?
        raise ReddishParser::ParserError.new(msg)
      else
        raise ReddishParser::UnexpectedKeyword.new(msg)
      end
    end

    private
    def dig_condition_command!(command)
      # Dig the head Element::Connector.
      # If type is Semicolon, set it to condition.
      # ex. if cmd1; cmd2 ; cmd3 && cmd4; end
      #        ~~~~
      #        this

      last_semicolon = nil
      current = command
      while current.class != Element::Command
        if current.type == :semicolon
          last_semicolon = current
        end
        current = current.cmd1
      end

      if last_semicolon
        condition_cmd = last_semicolon.cmd1
        last_semicolon.cmd1 = nil
        condition_cmd
      end
    end
  end
end

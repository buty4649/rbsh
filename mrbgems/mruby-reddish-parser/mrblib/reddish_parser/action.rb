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
        target = cmd1.class == Element::Pipeline ? cmd1.cmd2 : cmd2
        target.append_redirect(Element::Redirect.new(:copywrite, 2, 1, nil))
      end

      Element::Pipeline.new(cmd1, cmd2)
    end

    def on_connector(type, cmd1, cmd2)
      if type == :async
        cmd1.async = true
        Element::Connector.new(:semicolorn, cmd1, cmd2)
      else
        Element::Connector.new(type, cmd1, cmd2)
      end
    end

    def on_simple_list(connector, async)
      if async
        if connector.class == Element::Connector
          connector.cmd2.async = true
        else
          connector.async = true
        end
      end
      [connector]
    end

    def on_if_stmt(condition, cmd1, cmd2=nil)
      Element::IfStatement.new(:if, condition, cmd1, cmd2)
    end

    def on_error(msg)
      raise ReddishParser::ParserError.new(msg)
    end
  end
end

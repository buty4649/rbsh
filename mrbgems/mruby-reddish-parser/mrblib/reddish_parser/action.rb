module ReddishParser
  class Action
    def initialize(src)
      @__lexer = Lexer.new(src)
    end

    def on_word(word, before=nil)
      raise NotImplemented.new
    end

    def on_redirect(type, src, *args)
      raise NotImplemented.new
    end

    def on_redirect_list(redirect, before=nil)
      raise NotImplemented.new
    end

    def on_command(data, redirect=nil)
      raise NotImplemented.new
    end

    def on_command_list(cmd, async=false)
    end

    def on_pipeline(cmd1, cmd2)
      raise NotImplemented.new
    end

    def connector(type, cmd1, cmd2)
      raise NotImplemented.new
    end
  end
end

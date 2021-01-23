module ReddishParser
  # defined in parser.y
  #def parser(line); end

  # defined in parser.y
  #def debug=(bool); end

  class << self
    def lexer_debug
      @__debug ||= false
    end

    def lexer_debug=(v)
      @__debug = v.!.!
    end
  end
end

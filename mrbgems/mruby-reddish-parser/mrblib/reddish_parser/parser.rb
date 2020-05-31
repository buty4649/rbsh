module ReddishParser
  class Parser
    def initialize(line)
      @line = line
      @last_token = nil
      @token_before_that = nil
      @two_tokens_ago = nil

      separator = ENV['IFS'] || " \t\n"
      @separator_regexp = Regexp.new(separator.split('').join('|'))
    end

    def get_token
      token = read_token

      @two_tokens_ago = @token_before_that
      @token_before_that = token.type

      token
    end

    def read_token
      case c
      when nil then nil_token
      when '<' then lt_token
      when '>' then gt_token
      when '&' then and_token
      when '|' then or_token
      when @separator_regexp then separator_token
      else
        if m = match(/^(\d+)([<>])/)
          number = slice!(0...m.begin(2))
          Token.new(number, TokenType::NUMBER)
        elsif is_the_before_token_redirect? && m = match(/^((-)|(\d+)-|(\d+))/)
          type = if m[2]
                   TokenType::MINUS
                 elsif m[3]
                   TokenType::NUMBER_MINUS
                 elsif m[4]
                   TokenType::NUMBER
                 end
          Token.new(slice!(0..m.end(0)), type)
        else
          Token.new(read_word, TokenType::WORD)
        end
      end
    end

    def nil_token
      getc
      Token.new(nil, TokenType::YYEOF)
    end

    def lt_token
      getc
      Token.new(nil, TokenType::LT)
    end

    def gt_token
      getc
      if c == '>'
        getc
        Token.new(nil, TokenType::GT_GT)
      else
        Token.new(nil, TokenType::GT)
      end
    end

    def and_token
      getc
      if c == '&'
        getc
        Token.new(nil, TokenType::AND_AND)
      else
        Token.new(nil, TokenType::AND)
      end
    end

    def or_token
      getc
      if c == '|'
        getc
        Token.new(nil, TokenType::OR_OR)
      else
        Token.new(nil, TokenType::OR)
      end
    end

    def separator_token
      getc while separator?
      Token.new(Word.new(nil, WordType::SEPARATOR), TokenType::WORD)
    end

    def read_word
      case c
      when "'" then quote_word("'", WordType::QUOTE)
      when '"' then quote_word('"', WordType::DQOUTE)
      else normal_word
      end
    end

    def quote_word(quote, type)
      getc
      i = index(quote)
      if i.nil?
        error("unterminated string")
      end
      str = slice!(0...i)
      getc
      Word.new(str, type)
    end

    def normal_word
      regexp = Regexp.new([@separator_regexp.to_s, '"', "'"].join('|'))
      i = index(regexp) || length

      # check redirection word
      r = index(/[<>]/)
      i = r if r && r < i

      Word.new(slice!(0...i), WordType::NORMAL)
    end

    def c(i=0)
      @line[i]
    end

    def length
      @line.length
    end

    def index(p)
      s = 0
      while i = @line.index(p, s)
        break if c(i-1) != "\\"
        s = i + 1
      end
      i
    end

    def getc
      slice!(0)
    end

    def slice!(r)
      @line.slice!(r)
    end

    def match(m)
      m.match(@line)
    end

    def separator?
      @separator_regexp.match(c)
    end

    def is_the_before_token_redirect?
      [TokenType::LT, TokenType::GT].include?(@two_tokens_ago) && @token_before_that == TokenType::AND
    end

    def error(str)
      raise ParserError.new(str)
    end
  end
end

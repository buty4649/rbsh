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
      when ';' then semicolorn_token
      when @separator_regexp then separator_token
      else
        if m = match(/^(\d+)([<>])/)
          number = slice!(0...m.begin(2))
          Element::Token.new(number, TokenType::NUMBER)
        elsif is_the_before_token_redirect? && m = match(/^((-)|(\d+)-|(\d+))/)
          type = if m[2]
                   TokenType::MINUS
                 elsif m[3]
                   TokenType::NUMBER_MINUS
                 elsif m[4]
                   TokenType::NUMBER
                 end
          Element::Token.new(slice!(0..m.end(0)), type)
        elsif match(/^%!/)
          getc
          Element::Token.new(quote_word("!", WordType::DQOUTE), TokenType::WORD)
        elsif m = match(/^%([qQ])(\p{Punct}|[<>\|])/)
          getc; getc
          type = {"q" => WordType::QUOTE, "Q" => WordType::DQOUTE}[m[1]]
          if type.nil?
            error("unknown type or not implimented of %string")
          end
          paren = {"(" => ")", "[" => "]", "{" => "}", "<" => ">"}[m[2]]
          paren ||= m[2]
          Element::Token.new(quote_word(paren, type), TokenType::WORD)
        else
          Element::Token.new(read_word, TokenType::WORD)
        end
      end
    end

    def nil_token
      getc
      Element::Token.new(nil, TokenType::YYEOF)
    end

    def lt_token
      getc
      if c == '>'
        getc
        Element::Token.new(nil, TokenType::LT_GT)
      elsif c == '&'
        getc
        Element::Token.new(nil, TokenType::LT_AND)
      else
        Element::Token.new(nil, TokenType::LT)
      end
    end

    def gt_token
      getc
      if c == '>'
        getc
        Element::Token.new(nil, TokenType::GT_GT)
      elsif c == '&'
        getc
        Element::Token.new(nil, TokenType::GT_AND)
      else
        Element::Token.new(nil, TokenType::GT)
      end
    end

    def and_token
      getc
      if c == '&'
        getc
        Element::Token.new(nil, TokenType::AND_AND)
      elsif c == '>'
        getc
        Element::Token.new(nil, TokenType::AND_GT)
      else
        Element::Token.new(nil, TokenType::AND)
      end
    end

    def or_token
      getc
      if c == '|'
        getc
        Element::Token.new(nil, TokenType::OR_OR)
      else
        Element::Token.new(nil, TokenType::OR)
      end
    end

    def semicolorn_token
      getc
      Element::Token.new(nil, TokenType::SEMICOLON)
    end

    def separator_token
      getc while separator?
      Element::Token.new(Element::Word.new(nil, WordType::SEPARATOR), TokenType::WORD)
    end

    def read_word
      case c
      when "'" then quote_word("'", WordType::QUOTE)
      when '"' then quote_word('"', WordType::DQOUTE)
      else normal_word
      end
    end

    def quote_word(paren, type)
      getc
      i = index(paren)
      if i.nil?
        error("unterminated string")
      end
      str = slice!(0...i)
      getc
      Element::Word.new(str, type)
    end

    def normal_word
      regexp = Regexp.new([@separator_regexp.to_s, '"', "'", ";", "&"].join('|'))
      i = index(regexp) || length

      # check redirection word
      r = index(/[<>]/)
      i = r if r && r < i

      Element::Word.new(slice!(0...i), WordType::NORMAL)
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
      [TokenType::LT_AND, TokenType::GT_AND].include?(@token_before_that)
    end

    def error(str)
      raise ParserError.new(str)
    end
  end
end

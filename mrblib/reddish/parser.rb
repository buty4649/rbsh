module Reddish
  class ParserError < Exception; end
  class Token < Struct.new(:word, :type); end

  class Parser

    def initialize(line)
      @line = line
    end

    def get_token
      case
      when c.nil?
        next!; Token.new(nil, TokenType::YYEOF)
      when blank?
        next! while blank?
        Token.new(Word.new(nil, WordType::SPLIT), TokenType::WORD)
      when c == '<'
        next!
        Token.new(nil, '<'.ord)
      when c == '>'
        next!
        if c == '>'
          next!
          Token.new(nil, TokenType::GREATER_GREATER)
        else
          Token.new(nil, '>'.ord)
        end
      when c == '&'
        next!
        if c == '&'
          next!
          Token.new(nil, TokenType::AND_AND)
        else
          Token.new(nil, '&'.ord)
        end
      when c == '|'
        next!
        if c == '|'
          next!
          Token.new(nil, TokenType::OR_OR)
        else
          Token.new(nil, '|'.ord)
        end
      else
        word = get_word
        Token.new(word, TokenType::WORD)
      end
    end

    def get_word
      case c
      when "'" then quote_word("'", WordType::QUOTE)
      when '"' then quote_word('"', WordType::DQOUTE)
      else normal_word
      end
    end

    def quote_word(quote, type)
      next!
      i = index(quote)
      if i.nil?
        error("unterminated string")
      end
      str = slice!(0...i)
      next!
      Word.new(str, type)
    end

    def normal_word
      i = index(/[ \t'"]/) || length

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

    def next!
      slice!(0)
    end

    def slice!(r)
      @line.slice!(r)
    end

    def blank?
      c == " " || c == "\t"
    end

    def error(str)
      raise ParserError.new(str)
    end
  end
end

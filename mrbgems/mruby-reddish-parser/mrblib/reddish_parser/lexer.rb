module ReddishParser
  class Lexer
    def initialize(line)
      @line = line.dup
      @last_token = nil
      @token_before_that = nil

      separator = ENV['IFS'] || " \t\n"
      @separator_regexp = Regexp.new(separator.split('').join('|'))
    end

    def get_token
      token = read_token
      @token_before_that = token.first
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
          [:number, number]
        elsif c == "-" && (is_the_before_token_redirect? || is_the_before_token_number?)
          getc
          [:"-"]
        elsif is_the_before_token_redirect? && m = match(/^(\d+)/)
          [:number, slice!(0...m.end(1))]
        elsif match(/^%!/)
          getc
          [:word, quote_word("!", :dquote)]
        elsif m = match(/^%([qQ])(\p{Punct}|[<>\|])/)
          getc; getc
          type = {"q" => :quote, "Q" => :dquote}[m[1]]
          if type.nil?
            error("unknown type or not implimented of %string")
          end
          paren = {"(" => ")", "[" => "]", "{" => "}", "<" => ">"}[m[2]]
          paren ||= m[2]
          [:word, quote_word(paren, type)]
        else
          [:word, read_word]
        end
      end
    end

    def nil_token
      getc
      [:eof]
    end

    def lt_token
      getc
      if c == '>'
        getc
        [:"<>"]
      elsif c == '&'
        getc
        [:"<&"]
      else
        [:"<"]
      end
    end

    def gt_token
      getc
      if c == '>'
        getc
        [:">>"]
      elsif c == '&'
        getc
        [:">&"]
      else
        [:">"]
      end
    end

    def and_token
      getc
      if c == '&'
        getc
        [:"&&"]
      elsif c == '>'
        getc
        [:"&>"]
      else
        [:"&"]
      end
    end

    def or_token
      getc
      if c == '|'
        getc
        [:"||"]
      elsif c == '&'
        getc
        [:"|&"]
      else
        [:"|"]
      end
    end

    def semicolorn_token
      getc
      [:";"]
    end

    def separator_token
      getc while separator?
      [:word, [:separator, nil]]
    end

    def read_word
      case c
      when "'" then quote_word("'", :quote)
      when '"' then quote_word('"', :dquote)
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
      [type, str]
    end

    def normal_word
      regexp = Regexp.new([@separator_regexp.to_s, '"', "'", ";", "&"].join('|'))
      i = index(regexp) || length

      # check redirection word
      r = index(/[<>]/)
      i = r if r && r < i

      [:normal, slice!(0...i)]
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
      [:"<&", :">&"].include?(@token_before_that)
    end

    def is_the_before_token_number?
      @token_before_that == :number
    end

    def error(str)
      raise ParserError.new(str)
    end
  end
end

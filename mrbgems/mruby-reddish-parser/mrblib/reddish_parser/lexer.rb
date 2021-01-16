module ReddishParser
  class Lexer
    # '<', '<>', '<&', '>', '>>', '>&', '&', '&&', '&>', '|', '||', '|&', ';'
    SIMPLE_TOKEN_PATTERN = '([<>&][>&]?|\|[|&]?|;)'
    QUOTE_WORD_PATTERN = %Q!["']!
    PERCENT_WORD_PATTERN = '(%(!|[qQ]\W))'

    def initialize(line)
      @line = line.dup
      @last_token = nil

      separator = ENV['IFS'] || " \t\n"
      @separator_pattern = "[#{separator}]"
    end

    def get_token
      if @last_token.nil? || @last_token != :word
         read_sperator
      end

      token = eof_token       ||
              simple_token    ||
              separator_token ||
              number_token    ||
              hyphen_token    ||
              word_token
      @last_token = token.type
      token
    end

    def eof_token
      if @line.nil? || @line.empty?
        return Token.new(:eof)
      end
    end

    def simple_token
      token = @line.slice!(/^#{SIMPLE_TOKEN_PATTERN}/)
      return unless token
      Token.new(token.to_sym)
    end

    def separator_token
      if read_sperator
        Token.new(:word, [:separator])
      end
    end

    def read_sperator
      @line.slice!(/^#{@separator_pattern}+/)
    end

    def number_token
      if number = @line.slice!(/^\d+(?=[<>])/) ||
        ([:"<&", :">&"].include?(@last_token) && number = @line.slice!(/^\d+/))
        Token.new(:number, number)
      end
    end

    def hyphen_token
      if [:"<&", :">&", :number].include?(@last_token) && hyphen = @line.slice!("-")
        Token.new(hyphen.to_sym)
      end
    end

    def word_token
      word = quote_word   ||
             percent_word ||
             normal_word
      Token.new(:word, word)
    end

    def quote_word
      if s = @line.slice!(/^#{QUOTE_WORD_PATTERN}/)
        type = s == '"' ? :dquote : :quote
        [type, read_quote_word(s)]
      end
    end

    def percent_word
      if s = @line.slice!(/^#{PERCENT_WORD_PATTERN}/)
        _, quote, paren = s.split("")
        if quote == "!"
          quote = "Q"
          paren = "!"
        end
        type = {"Q" => :dquote, "q" => :quote}[quote]
        term = {"(" => ")", "[" => "]", "{" => "}", "<" => ">"}[paren] || paren
        [type, read_quote_word(term)]
      end
    end

    def read_quote_word(term)
      t = Regexp.escape(term)
      word = @line.slice!(/^.+?(?<!\\)#{t}/)
      raise SyntaxError.new("unterminated string") unless word
      word.delete_suffix!(term).gsub(/\\#{t}/, term)
    end

    def normal_word
      pattern = [SIMPLE_TOKEN_PATTERN, QUOTE_WORD_PATTERN, PERCENT_WORD_PATTERN, @separator_pattern].join("|")
      pos = @line.index(/(?<!\\)(#{pattern})/)
      pos = pos ? pos - 1 : -1
      [:normal, @line.slice!(0..pos)]
    end
  end
end

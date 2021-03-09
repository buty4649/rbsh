module ReddishParser
  class Lexer

    # '<', '<>', '<&', '>', '>>', '>&', '&', '&&', '&>', '&>>', '>>&', '|', '||', '|&', ';'
    SIMPLE_TOKEN_PATTERN = '(&>>|>>&|[<>&][>&]?|\|[|&]?|;)'
    QUOTE_WORD_PATTERN = %Q!["'`]!
    PERCENT_WORD_PATTERN = '(%(!|[qQ]\W))'

    def initialize(line)
      @line = line.dup
      @last_token = nil
      @statement = 0

      if ReddishParser.lexer_debug
        STDERR.puts "lexer: Input line:#{@line.gsub(/\n/, "\\n")}"
      end
    end

    def get_token
      if @last_token.nil? || @last_token != :word
        read_separator(@last_token.nil?)
      end

      token = newline_token   ||
              eof_token       ||
              simple_token    ||
              number_token    ||
              hyphen_token    ||
              keyword_token   ||
              word_token

      if token.type == :word && @last_token == :for
        read_separator(false)
      end

      @last_token = token.type
      if [:";", :"\n", :"|", :"|&"].include?(token.type) && @statement.zero?
        @last_token = nil
      end

      if ReddishParser.lexer_debug
        type = token.type
        if type == :"\n"
          type = "newline"
        elsif type == :word
          type = "word #{token.data.first} '#{token.data.last}'"
        end
        STDERR.puts "lexer:   Token type: #{type} line: #{@line}"
      end

      token
    end

    def eof_token
      if @line.nil? || @line.empty?
        return Token.new(:eof)
      end
    end

    def newline_token
      # remove escaped newline
      @line.slice!(/\A\\\n/)

      if token = @line.slice!(/\A\n+/)
        Token.new(token[0].to_sym)
      end
    end

    def simple_token
      token = @line.slice!(/\A#{SIMPLE_TOKEN_PATTERN}/)
      return unless token
      Token.new(token.to_sym)
    end

    def number_token
      if number = @line.slice!(/\A\d+(?=[<>])/) ||
        ([:"<&", :">&"].include?(@last_token) && number = @line.slice!(/\A\d+/))
        Token.new(:number, number)
      end
    end

    def hyphen_token
      if [:"<&", :">&", :number].include?(@last_token) && @line.slice!("-")
        Token.new(:"-")
      end
    end

    def keyword_token
      return if @last_token && @statement.zero?

      if k = @line.slice!(/\A(if|unless|while|until|for)(?=#{separator_pattern}|\z)/)
        @statement += 1
        Token.new(k.to_sym)
      elsif k = @line.slice!(/\A(fi|done|end|})(?=#{separator_pattern})?/)
        @statement -= 1
        Token.new(k.to_sym)
      elsif k = @line.slice!(/\A(do|then|el(se|s?if)|in|{)(?=#{separator_pattern})?/)
        Token.new(k.to_sym)
      end
    end

    def word_token
      word = separator    ||
             quote_word   ||
             exec_word    ||
             percent_word ||
             normal_word
      Token.new(:word, word)
    end

    def separator
      if read_separator
        [:separator]
      end
    end

    def quote_word
      if s = @line.slice!(/\A#{QUOTE_WORD_PATTERN}/)
        type = case s
               when '"' then :normal
               when "'" then :quote
               when '`' then :execute
               end
        [type, read_quote_word(s)]
      end
    end

    def exec_word
      if s = @line.slice!(/\A\$\(/)
        [:execute, read_quote_word(")")]
      end
    end

    def percent_word
      if s = @line.slice!(/\A#{PERCENT_WORD_PATTERN}/)
        _, quote, paren = s.split("")
        if quote == "!"
          quote = "Q"
          paren = "!"
        end
        type = {"Q" => :normal, "q" => :quote}[quote]
        term = {"(" => ")", "[" => "]", "{" => "}", "<" => ">"}[paren] || paren
        [type, read_quote_word(term)]
      end
    end

    def read_quote_word(term)
      t = Regexp.escape(term)
      word = @line.slice!(/\A.*?(?<!\\)#{t}/m)
      raise UnterminatedString.new unless word
      word.delete_suffix!(term).gsub(/\\#{t}/, term)
    end

    def normal_word
      pattern = [SIMPLE_TOKEN_PATTERN, QUOTE_WORD_PATTERN, PERCENT_WORD_PATTERN, separator_pattern].join("|")
      pos = @line.index(/(?<!\\)(#{pattern})/)
      pos = pos ? pos - 1 : -1
      [:normal, @line.slice!(0..pos)]
    end

    def read_separator(newline=true)
      @line.slice!(/\A#{separator_pattern(newline)}+/)
    end

    def separator_pattern(newline=true)
      newline ? "[ \t\n]" : "[ \t]"
    end

    def state
      [@statement]
    end
  end
end

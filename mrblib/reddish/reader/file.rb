module Reddish::Reader
  class File < Base

    def initialize(file, show_prompt)
      @file = file
      @show_prompt = show_prompt
      @lineno = 0
    end

    def readline(prompt)
      if @show_prompt
        STDOUT.write(prompt)
      end

      line = @file.readline
      @lineno += 1
      if @lineno == 1 && line.index("#!") == 0
        # skip shebang
        line = @file.readline
      end
      line.chomp
    rescue EOFError => e
      nil
    end
  end
end

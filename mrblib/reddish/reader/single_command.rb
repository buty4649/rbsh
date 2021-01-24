module Reddish::Reader
  class SingleCommand < Base
    def initialize(command)
      @line = command
    end

    def readline(prompt)
      result = @line
      @line = nil
      result
    end
  end
end

module Reddish
  class Variable
    def initialize
      @local_variables = {}
    end

    def [](key)
      @local_variables[key] || ENV[key]
    end

    def []=(key, value)
      if ENV.key?(key)
        ENV[key] = value
      else
        @local_variables[key] = value
      end
    end

    def export(key)
      ENV[key] = @local_variables.delete(key) || ""
    end

    def unexport(key)
      @local_variables[key] = ENV.delete(key) || ""
    end
  end
end

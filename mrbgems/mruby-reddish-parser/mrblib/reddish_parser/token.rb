module ReddishParser
  class Token
    attr_reader :type, :data

    def initialize(type, data = nil)
      @type = type
      @data = data
    end
  end
end

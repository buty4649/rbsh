module ReddishParser
  module Element
    class Word
      attr_reader :type, :string

      def initialize(data)
        @type = data.first
        @string = data.last
      end

      alias :to_s :string
    end
  end
end

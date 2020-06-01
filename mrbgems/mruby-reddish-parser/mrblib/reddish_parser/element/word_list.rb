module ReddishParser
  module Element
    class WordList

      def initialize(word)
        # Array<Array<Word>>
        @list = [[]]
        add(word)
      end

      def add(word)
        if word.type == WordType::SEPARATOR
          @list.push([]) unless @list.last.empty?
        else
          @list.last.push(word)
        end
        self
      end

      def length
        @list.length
      end

      def [](range)
        @list[range].map(&:to_s).join
      end

      def first
        self[0]
      end

      # @return Array<Word>
      def to_a
        @list.select{|a| a.empty?.! }.map{|w| w.map(&:to_s).join }
      end

      def to_s
        self.to_a.join
      end
    end
  end
end

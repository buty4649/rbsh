module Reddish
  module Element
    class WordList

      def initialize(word)
        # Array<Array<Word>>
        @list = [[]]
        add(word)
      end

      def add(word)
        if word.type == :separator
          @list.push([]) unless @list.last.empty?
        else
          @list.last.push(word)
        end
        self
      end
      alias :"<<" :add

      # @return Array<String>
      def to_a
        @list.select{|a| a.empty?.! }.map{|w| w.map(&:to_s).join }
      end

      def to_s
        self.to_a&.join
      end
    end
  end
end

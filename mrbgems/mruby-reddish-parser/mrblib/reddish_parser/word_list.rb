module ReddishParser
  class WordList

    def initialize(word)
      @list = [word]
    end

    def add(word)
      @list.push(word)
    end

    def shift
      loop do
        return nil if @list.first.nil?
        break if @list.first.type != WordType::SPLIT
        @list.shift
      end

      result = WordList.new(@list.shift)
      while @list.first && @list.first.type != WordType::SPLIT
        result.add(@list.shift)
      end

      result
    end

    def to_a
      result = []
      while wl = shift
        result << wl
      end
      result
    end

    def to_s
      @list.map(&:to_s).join
    end

  end
end

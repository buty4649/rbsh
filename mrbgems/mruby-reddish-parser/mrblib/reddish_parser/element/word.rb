module ReddishParser
  module Element
    class Word < Struct.new(:content, :type)

      def to_s
        if self[:type] == WordType::QUOTE
          return self[:content]
        end

        if self[:type] == WordType::SEPARATOR
          return ''
        end

        s = self[:content].gsub(/\${(\w+)}/) { ENV[$1] || "" }
        s = s.gsub(/\$(\w+)/) { ENV[$1] || "" }
      end

    end
  end
end

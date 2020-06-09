module Reddish
  module Element
    class Word < Struct.new(:type, :content)

      def to_s
        if self[:type] == :quote
          return self[:content]
        end

        if self[:type] == :separator
          return ''
        end

        s = self[:content].gsub(/\${(\w+)}/) { ENV[$1] || "" }
        s = s.gsub(/\$(\w+)/) { ENV[$1] || "" }
      end

    end
  end
end

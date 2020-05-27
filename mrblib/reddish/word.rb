module Reddish

  class WordType
    %w(
      NORMAL QUOTE DQOUTE SPLIT
    ).each do |const|
      self.const_set(const.to_sym, const)
    end
  end

  class Word < Struct.new(:content, :type)

    def to_s
      if self[:type] == WordType::QUOTE
        return self[:content]
      end

      s = self[:content].gsub(/\${(\w+)}/) { ENV[$1] || "" }
      s = s.gsub(/\$(\w+)/) { ENV[$1] || "" }
    end

  end
end

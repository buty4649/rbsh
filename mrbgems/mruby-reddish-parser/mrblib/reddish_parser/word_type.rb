module ReddishParser
  class WordType
    %w(
      NORMAL QUOTE DQOUTE SPLIT
    ).each do |const|
      self.const_set(const.to_sym, const)
    end
  end
end

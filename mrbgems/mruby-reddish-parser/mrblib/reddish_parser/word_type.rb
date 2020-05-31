module ReddishParser
  class WordType
    %w(
      NORMAL NORMAL QUOTE DQOUTE SEPARATOR
    ).each do |const|
      self.const_set(const.to_sym, const)
    end
  end
end

module ReddishParser
  class ConnectorType
    %w(AND OR).each do |const|
      self.const_set(const.to_sym, const)
    end
  end
end

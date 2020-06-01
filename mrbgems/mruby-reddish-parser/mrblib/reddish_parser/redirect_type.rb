module ReddishParser
  class RedirectType
    %w(
      APPEND CLOSE
      COPYREAD COPYWRITE
      READWRITE READ WRITE
    ).each do |const|
      self.const_set(const.to_sym, const)
    end
  end
end

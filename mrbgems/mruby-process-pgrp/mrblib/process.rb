module Process
  def self.getpgrp
    getpgid(0)
  end

  def self.setpgrp
    setpgid(0, 0)
  end
end

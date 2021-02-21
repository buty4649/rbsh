module Ruby
  def self.exec_from_file(file, *argv)
    code = file == "-" ? STDIN.read : File.read(file)
    exec(code, file, *argv)
  end
end

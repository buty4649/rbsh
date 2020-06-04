def __main__(argv)
  result = Dir.glob("/proc/#{$$}/fd/*").map do |path|
    fd = File.basename(path)
    begin
      realpath = File.readlink(path)
      [fd, realpath]
    rescue Errno::ENOENT
    end
  end

  if argv[1]
    File.open(argv[1], "w") do |f|
      f.puts result.compact.to_h.to_json
    end
  else
    begin
      puts result.compact.to_h.to_json
    rescue Errno::EBADF
      STDERR.puts result.compact.to_h.to_json
    end
  end
end

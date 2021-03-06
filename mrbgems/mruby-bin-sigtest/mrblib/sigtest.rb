def __main__(argv)
  puts "sigtest wait"
  Signal.trap(:INT) do
    File.open(argv[1] || "/dev/stdout", "a+") do |f|
      f.puts "SIGINT"
    end
    exit(0)
  end

  sleep 10
end

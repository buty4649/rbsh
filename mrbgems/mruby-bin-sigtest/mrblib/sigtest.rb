def __main__(argv)
  puts "sigtest wait"
  SignalThread.trap(:INT) do
    File.open(argv[1], "a+") do |f|
      f.puts "SIGINT"
    end
    exit(0)
  end

  sleep 10
end

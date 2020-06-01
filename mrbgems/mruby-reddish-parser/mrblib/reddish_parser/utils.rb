module ReddishParser
  class Utils
    def self.search_command(command)
      return command if command.include?("/")

      result = ENV["PATH"].split(/:/)
                          .map {|dir| "#{dir}/#{command}" }
                          .find {|path| File.exists?(path) }
      return result || command
    end
  end
end

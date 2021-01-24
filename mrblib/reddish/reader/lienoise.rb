module Reddish::Reader
  class Linenoise < Base

    def initialize(data_home)
      @history_file_path = ::File.join(data_home, "history.txt")

      ::Linenoise.multi_line = true
      if ::File.exists?(@history_file_path)
        ::Linenoise::History.load(@history_file_path)
      end
      completion
    end

    def readline(prompt)
      linenoise(prompt)
    end

    def add_history(history)
      history.each do |cmd|
        ::Linenoise::History.add(cmd)
      end
      ::Linenoise::History.save(@history_file_path)
    end

    def completion
      ::Linenoise.completion do |buf|
        if buf.index(".")&.zero?
          Dir.glob("#{buf}*").to_a
        else
          result = ENV["PATH"].split(/:/).map do |path|
            Dir.glob(::File.join(path, "#{buf}*")).to_a
               .select{|file| ::File::Stat.new(file).executable? }
               .map{|file| file.delete_prefix("#{path}/") }
          end
          result.flatten.compact.sort.uniq
        end
      end
    end
  end
end

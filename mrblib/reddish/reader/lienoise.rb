module Reddish::Reader
  class Linenoise < Base

    def initialize(data_home)
      @history_file_path = ::File.join(data_home, "history.txt")

      ::Linenoise.multi_line = true
      if ::File.exists?(@history_file_path)
        ::Linenoise::History.load(@history_file_path)
      end
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
  end
end

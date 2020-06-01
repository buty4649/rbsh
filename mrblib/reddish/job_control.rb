module Reddish
  class JobControl
    def run(command_list)
      command_list.each(&:exec)
    end
  end
end

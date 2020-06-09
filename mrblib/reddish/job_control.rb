module Reddish
  class JobControl
    def run(command_list)
      th = SignalThread.trap(:INT, {detailed: true}) do |info|
        pids.each {|pid| Process.kill(:INT, pid) }
      end
      command_list.each(&:exec)
      th.cancel
    end

    def self.start_sigint_trap(pids, &block)
      th = SignalThread.trap(:INT, {detailed: true}) do |info|
        pids.each {|pid| Process.kill(:INT, pid)}
      end
      result = block.call
      th.cancel
      result
    end
  end
end

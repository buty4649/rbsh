class JobControl
  def run(executor, command_list)
    command_list.each do |command|
      executor.exec(command)
      executor.reset
    end
  end

  def self.start_sigint_trap(pids, &block)
    #th = SignalThread.trap(:INT, {detailed: true}) do |info|
    #  pids.each {|pid| Process.kill(:INT, pid)}
    #end
    result = block.call
    #th.cancel
    result
  end
end

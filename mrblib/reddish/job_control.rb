class JobControl
  def run(executor, command_list)
    command_list.each do |command|
      executor.exec(command)
      executor.reset
    end
  end

  def self.start_sigint_trap(pgid, &block)
    st = SignalTrap.new
    st.start_sigint_trap(pgid)
    result = block.call
    st.stop_trap
    result
  end
end

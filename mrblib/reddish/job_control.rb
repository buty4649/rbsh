class JobControl
  def run(executor, command)
    executor.exec(command)
    executor.reset
  end

  def self.start_sigint_trap(pgid, &block)
    st = SignalTrap.new
    st.start_sigint_trap(pgid)
    result = block.call
    st.stop_trap
    result
  end
end

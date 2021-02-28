class JobControl
  def run(executor, command)
    executor.exec(command)
    executor.reset
  end
end

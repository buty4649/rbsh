class SystemExit < Exception
  attr_reader :status
  def initialize(status=0, error_message="")
    @status = status
    @message = message
  end

  def success?
    @status.zero?
  end
end

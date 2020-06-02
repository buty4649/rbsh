module ReddishParser
  module Element
    class Connector < Struct.new(:type, :cmd1, :cmd2)
      def exec
        result = self[:cmd1].exec

        if cmd2_exec?(result)
          result = self[:cmd2].exec
        end

        result
      end

      def cmd2_exec?(r)
        t = self[:type]
        (t == ConnectorType::AND && r.success?)   ||
        (t == ConnectorType::OR  && r.success?.!) ||
        t == ConnectorType::SEMICOLON
      end
    end
  end
end

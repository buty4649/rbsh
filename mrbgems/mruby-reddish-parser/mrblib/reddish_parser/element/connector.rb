module ReddishParser
  module Element
    class Connector < Struct.new(:type, :cmd1, :cmd2)
      def exec
        result = self[:cmd1].exec

        if result.success?
          if self[:type] == ConnectorType::AND
            result = self[:cmd2].exec
          end
        elsif self[:type] == ConnectorType::OR
          result = self[:cmd2].exec
        end

        result
      end
    end
  end
end

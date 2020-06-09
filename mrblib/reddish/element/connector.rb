module Reddish
  module Element
    class Connector < Struct.new(:type, :cmd1, :cmd2)
      def exec
        if self[:type] == :async
          self[:cmd1].async = true
        end

        result = self[:cmd1].exec

        if cmd2_exec?(result)
          result = self[:cmd2].exec
        end

        result
      end

      def cmd2_exec?(r)
        t = self[:type]
        (t == :and && r.success?)   ||
        (t == :or  && r.success?.!) ||
        t == :semicolon ||
        t == :async
      end
    end
  end
end

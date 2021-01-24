module ReddishParser
  module Element
    class Command
      attr_reader :env, :cmdline, :redirect
      attr_accessor :async

      def initialize(cmdline, redirect)
        @env, @cmdline = split(cmdline)
        @redirect = redirect
        @async = false
      end

      def append_redirect(redirect)
        @redirect = [] if @redirect.nil?
        @redirect.push(redirect)
      end

      private
      def split(input)
        list = input.drop_while{|w| w.type == :separator}

        env = {}
        while c = list.first
          if c.type == :separator
            list.shift; next
          end
          break if c.type != :normal || c.to_s.index("=").nil?

          k, v = c.to_s.split(/=/, 2)
          break unless k =~ /^\w+$/
          list.shift
          env[k] = [Element::Word.new([:normal, v])]
          until list.empty? || list.first.type == :separator
            env[k] << list.shift
          end
        end

        if list.empty?
          return [env, nil]
        end

        cmdline=[[]]
        while c = list.shift
          if c.type == :separator
            cmdline.append([])
          else
            cmdline.last.append(c)
          end
        end

        [env, cmdline]
      end
    end
  end
end

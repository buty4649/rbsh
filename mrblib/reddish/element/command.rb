module Reddish
  module Element
    class Command
      attr :wordlist, :redirect, :async
      attr_accessor :async

      def initialize(wordlist, redirect=nil)
        @wordlist = wordlist
        @redirect = redirect || []
        @async = false
      end

      def add_redirect(redirect)
        if redirect.class == Array
          @redirect += redirect
        else
          @redirect <<= redirect
        end
      end
    end
  end
end

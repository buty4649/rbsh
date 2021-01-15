module ReddishParser
  module Element
    class Redirect
      attr_reader :type, :dest, :src, :filename

      def initialize(type, dest, src=nil, filename=nil)
        @type = type
        @dest = dest.to_i
        @src  = src&.to_i
        @filename = filename
      end
    end
  end
end

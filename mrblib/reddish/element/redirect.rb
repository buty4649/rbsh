module Reddish
  module Element
    class Redirect < Struct.new(:type, :dest_fd, :src_fd, :filename); end
  end
end

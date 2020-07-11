module Reddish
  module BuiltinCommands
    module Cd
      include Base

      def cd(*args)
        return error("Too many args") if args.length > 1

        path = args&.first
        if path.nil?
          unless dest = ENV["HOME"]
            return error('$HOME not set')
          end
        elsif path == "-"
          unless dest = ENV["OLDPWD"]
            return error('$OLDPWD not set')
          end
        else
          dest = path
        end

        ENV["OLDPWD"] = Dir.pwd
        Dir.chdir(dest)

        success
      end
    end
    extend Cd
  end
end

module Reddish
  module BuiltinCommands
    AVAILABLE_COMMANDS = %i(cd echo puts)

    def self.call(cmd, *args)
      if AVAILABLE_COMMANDS.include?(cmd)
        self.__send__(cmd.to_sym, *args)
      else
        self.error(cmd, "Not a shell builtin")
      end
    end

    def self.define_commands(dest)
      AVAILABLE_COMMANDS.each do |name|
        dest.define_command(name, -> (*args) { self.call(name, *args) })
      end

      dest.define_command(:builtin, -> (cmd, *args) { self.call(cmd.to_sym, *args) })
    end
  end
end

module Reddish
  module BuiltinCommand
    AVAILABLE_COMMANDS = %i(cd echo puts)

    def self.call(cmd, *args)
      if AVAILABLE_COMMANDS.include?(cmd)
        self.__send__(cmd.to_sym, *args)
      else
        self.error(cmd, "Not a shell builtin")
      end
    end

    def self.success
      Process::Status.new($$, 0)
    end

    def self.error(cmd, msg, status=1)
      STDERR.puts "reddish: #{cmd}: #{msg}"
      Process::Status.new($$, status)
    end

    def self.define_commands(dest)
      AVAILABLE_COMMANDS.each do |name|
        dest.define_command(name, -> (*args) { self.call(name, *args) })
      end

      dest.define_command(:builtin, -> (cmd, *args) { self.call(cmd.to_sym, *args) })
    end

    def self.cd(*args)
      return self.error("Too many args") if args.length > 1

      path = args&.first
      if path.nil?
        unless dest = ENV["HOME"]
          return self.error('$HOME not set')
        end
      elsif path == "-"
        unless dest = ENV["OLDPWD"]
          return self.error('$OLDPWD not set')
        end
      else
        dest = path
      end

      ENV["OLDPWD"] = Dir.pwd
      Dir.chdir(dest)

      self.success
    end

    BACKSLASH_REPLACE_TABLE = {
      t: "\t", v: "\v", n: "\n", r: "\r", f: "\f",
      b: "\b", a: "\a", e: "\e", s: "\s"
    }

    def self.echo(*args)
      class << args; include Getopts; end
      opts = args.getopts("eEns")
      str = args[(args.optind-1)..-1].join(opts["s"] ? "" : " ")

      str += "\n" if opts["n"].nil?

      if opts["e"]
        str = escape(str)
      end

      STDOUT.print str
      self.success
    end

    def self.puts(*args)
      args.each {|arg| STDOUT.puts escape(arg) }
      self.success
    end

    private
    def self.hexstr2unicode(str)
      s = str.sub(/^0+/, "")
      [s.to_i(16)].pack("U")
    end

    def self.escape(str)
      str.gsub(/\\[0-7]{3}/) {|m| m[1..3].to_i(8).chr }
         .gsub(/\\x[0-9a-zA-Z]{2}/) {|m| m[2..3].to_i(16).chr }
         .gsub(/\\u[0-9a-zA-Z]{1,4}/) {|m| hexstr2unicode(m[2..-1]) }
         .gsub(/\\U[0-9a-zA-Z]{1,8}/) {|m| hexstr2unicode(m[2..-1]) }
         .gsub(/\\u{[0-9a-zA-Z ]+}/)  {|m| m[3..-2].split(/ /).map{|s| hexstr2unicode(s)}.join }
         .gsub(/\\./) {|m| s = m[-1]; BACKSLASH_REPLACE_TABLE[s.to_sym] || s }
    end
  end
end

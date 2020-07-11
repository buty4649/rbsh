module Reddish
  module BuiltinCommands
    module Echo
      include Base

      BACKSLASH_REPLACE_TABLE = {
        t: "\t", v: "\v", n: "\n", r: "\r", f: "\f",
        b: "\b", a: "\a", e: "\e", s: "\s"
      }

      def echo(*args)
        # ignore invalid option
        if args.first !~ /^-[eEns]+$/
          opts = {}
          str = args.join(opts["s"] ? "" : " ")
        else
          opts, optind = getopts("echo", args, "eEns")
          return error("echo") if opts.nil?
          str = args[(optind-1)..-1].join(opts["s"] ? "" : " ")
        end

        str += "\n" if opts["n"].nil?

        if opts["e"]
          str = escape(str)
        end

        STDOUT.print str
        success
      end

      def hexstr2unicode(str)
        s = str.sub(/^0+/, "")
        [s.to_i(16)].pack("U")
      end

      def escape(str)
        str.gsub(/\\[0-7]{3}/) {|m| m[1..3].to_i(8).chr }
          .gsub(/\\x[0-9a-zA-Z]{2}/) {|m| m[2..3].to_i(16).chr }
          .gsub(/\\u[0-9a-zA-Z]{1,4}/) {|m| hexstr2unicode(m[2..-1]) }
          .gsub(/\\U[0-9a-zA-Z]{1,8}/) {|m| hexstr2unicode(m[2..-1]) }
          .gsub(/\\u{[0-9a-zA-Z ]+}/)  {|m| m[3..-2].split(/ /).map{|s| hexstr2unicode(s)}.join }
          .gsub(/\\./) {|m| s = m[-1]; BACKSLASH_REPLACE_TABLE[s.to_sym] || s }
      end
    end
    extend Echo
  end
end

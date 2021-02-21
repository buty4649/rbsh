module Reddish
  module BuiltinCommands
    module IRuby
      include Base

      def iruby(*args)
        opts, optind = getopts("iruby", args, "e:v")
        return error(iruby, "invalid option") if opts.nil?

        if opts["v"]
          puts "iruby use #{MRUBY_DESCRIPTION}"
          return success
        end

        ret = if code = opts["e"]
          Ruby.exec(code, "-e", *args[(optind-1)..-1])
        else
          a = *args[(optind-1)..-1]
          filename = a.shift || "-"
          Ruby.exec_from_file(filename, *a)
        end

        ret.zero? ? success : error("iruby", nil, ret)
      end
    end
    extend IRuby
  end
end

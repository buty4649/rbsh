module XDG
  DEFAULTS = {
    'XDG_DATA_HOME'   => '~/.local/share',
    'XDG_DATA_DIRS'   => '/usr/local/share:/usr',
    'XDG_CONFIG_HOME' => '~/.config',
    'XDG_CONFIG_DIRS' => '/etc/xdg',
    'XDG_CACHE_HOME'  => '~/.cache',
    'XDG_CACHE_DIRS'  => '/tmp',
  }

  def self.[](env)
    @__environment ||= {}
    xdg_key = env.index("XDG_") == 0 ? env : "XDG_#{env}"
    @__environment[xdg_key] ||= ENV[xdg_key] || DEFAULTS[xdg_key]
  end
end

[![CI](https://github.com/buty4649/rbsh/actions/workflows/ci.yaml/badge.svg)](https://github.com/buty4649/rbsh/actions/workflows/ci.yaml)

rbsh: Rubyish and Bash like syntax shell
========================================

**⚠現在、開発中のためバグや機能不足が存在します⚠**

rbshはRubyが使えるシェル実装です。[mruby](https://github.com/mruby/mruby)を使っているためワンバイナリーで利用できます。bashとの互換が一部あり、bashスクリプトを動かすこともできます。

コンセプト
----------

rbshにおけるコンセプトは以下の通りです。

* Rubyのような構文
* bashスクリプトが80%くらい動く
* mrubyを内蔵してRubyスクリプトも実行できる
* ワンバイナリーで動く

動作環境
---------

以下の環境で動作することを確認しています。

* Ubuntu 21.04

インストール方法
----------------

現在、ビルドされたバイナリの配布は行っていないため、ソースからビルドする必要があります。ビルドするためには、以下の環境を用意します。

* Ubuntu 21.04
* Rust 1.56.0
* Ruby 3.0.2
* rake
* curl
* git

環境を用意したら以下の手順を実行することでバイナリを作成することができます。

```
$ git clone https://github.com/buty4649/rbsh
$ cd rbsh
$ cargo build --release
$ ./target/release/rbsh
```

オプション
-----------

rbshを起動時に以下のオプションを設定することができます。

<dl>
  <dt>-c string<dt>
  <dd>stringをコマンドとして実行します。</dd>

  <dt>--version</dt>
  <dd>バージョン情報を表示します。</dd>
</dl>

シェルの文法
------------

### コマンドの実行

ブランク区切りの単語とリダイレクションを記述します。最初の単語は実行するコマンド名になり、そのあとの単語があればコマンドの引数となります。もし行頭が変数の代入(NAMR=VAR)で始まっていた場合、その変数を一時的に環境変数にした状態でコマンドを実行します。

### リダイレクト

リダイレクトとは実行するコマンドの入出力を制御する機能のことです。リダイレクトを使用するには、以下に示すリダイレクト演算子を利用します。リダイレクト演算子は、コマンドの前や途中、最後に書くことができます。また、左から順に処理されます。

<dl>
  <dt>[n]&lt;word</dt>
  <dd>wordに対応したファイルがオープンされ、ファイルディスクリプタn(指定がない場合は0)に対して入力のリダイレクトが行われます。ファイルが存在しない場合はエラーになります。</dd>

  <dt>word&gt;[n]</dt>
  <dd>wordに対応したファイルがオープンされ、ファイルディスクリプタn(指定がない場合は1)に対して出力のリダイレクトが行われ、出力された内容で上書きされます。ファイルが存在しない場合は、新規にファイルが作成されます。</dd>

  <dt>word&gt;&gt;[n]</dt>
  <dd>wordに対応したファイルがオープンされ、ファイルディスクリプタn(指定がない場合は1)に対して出力のリダイレクトが行われ、出力された内容を追記します。ファイルが存在しない場合は、新規にファイルが作成されます。</dd>

  <dt>&&gt;word, &gt;&word</dt>
  <dd>&gt; word 2&gt;&1と等価です。</dd>

  <dt>&&gt;&gt;word, &gt;&gt;&word</dt>
  <dd>&gt;&gt; word 2&gt;&1 と等価です。</dd>

  <dt>[n]&lt;&digit, [n]&lt;&digit-, [n]&lt;&-</dt>
  <dd>入力ファイルディスクリプタをコピーします。ファイルディスクリプタn(指定がない場合は0)をdigitで指定されたファイルディスクリプタにコピーします。digitのあとに-が指定された場合は、コピーを行ったあとにnが閉じられます。-のみ指定された場合はnを閉じます。</dd>

  <dt>[n]&gt;&digit, [n]&gt&digit-, [n]&gt;&-</dt>
  <dd>出力ファイルディスクリプタをコピーします。ファイルディスクリプタn(指定がない場合は1)をdigitで指定されたファイルディスクリプタにコピーします。digitのあとに-が指定された場合は、コピーを行ったあとにnが閉じられます。-のみ指定された場合はnを閉じます。</dd>

  <dt>[n]&lt;&gt;word</dt>
  <dd>wordに対応したファイルが読み書きモードでオープンされ、ファイルディスクリプタn(指定がない場合は0)に対してリダイレクトされます。ファイルが存在しない場合新規に作成されます。</dd>
</dl>

### パイプライン

書式:

```
command1 | command2
command1 |& command2
```

command1の標準出力がcommand2の標準入力に接続されます。`|&` は `command1 2>&1 | command2` と等価です。

### コマンドの接続

書式:

```
command1 && command2
command1 || command2
```

`&&`の場合command1の終了ステータスが0の場合にcommand2が実行されます。`||`の場合、command1の終了ステータスが0以外の場合にcommand2が実行されます。

### if文

書式:

```
if condition1 [;] [then]
  commands1
[elif condition2 [;] [then]
  commands2 ]
[else commands3 ]
end

# bash形式
if condition1 ; then
  commands1
[elsif condtion2 ; then
  commands2]
[else commands3]
fi
```

condition1に指定されたコマンドを実行し最後に実行されたコマンドの終了ステータスが0の場合commands1を実行します。終了ステータスが0以外でelifまたはelseが指定されていた場合は、それぞれの処理に移ります。elif/elsifが指定されている場合、condition2に指定されたコマンドを実行し終了ステータスを確認します。終了ステータスが0の場合commands2を実行します。elseの場合はcommands3を実行します。

### unless文

書式:

```
unless condition1 [;] [then]
  commands1
[else command2]
end
```

条件が逆のif文です。

### while文

書式:

```
while condition [;] [do]
  commands
end

# bash形式
while condition ; do
  commands
done
```

conditionに指定されたコマンドを実行し最後に実行されたコマンドの終了ステータスが0の場合commandsを実行します。commandsの実行が終わったら再度conditionを評価し、終了ステータスが0の場合再度commandsを実行します。

### until文

書式:

```
until condition [;] [do]
  commands
end

# bash形式
until condition ; do
  commands
done
```

条件が逆のwhile文です。

### for文

書式:

```
for name [in wordlist] [;]
  commands
end

# bash形式
for name [in wordlist] ; do
  commands
done
```

wordlistを空白で分割した要素を順番にname変数に格納しcommandsを実行します。in以降を省略した場合$@が参照されます。

ビルトインコマンド
------------------

### break [n]

while/until/for文のループから脱出します。nが指定されない場合すべてのループから脱出します。while/until/for文の外で実行された場合エラーになります。

### continue [n], next [n]

while/until/for文の先頭に戻ります。nが指定されている場合、n個分のループを上がりループの先頭に戻ります。while/until/for文の外で実行された場合エラーになります。

### cd [arg]

カレントディレクトリをargに変更します。wordを指定しなかった場合は$HOMEに変更します。

### echo [arg...]

argを空白区切りで出力し最後に改行コードを出力します。終了コードは常に0です。

オプション:
<dl>
  <dt>-e</dt>
  <dd>エスケープ文字を解釈します。</dd>

  <dt>-n</dt>
  <dd>改行コードを出力しません。</dd>

  <dt>-s</dt>
  <dd>argの区切り文字をなくし出力します。</dd>
</dl>

### exit [n]

終了ステータスnで直ちにrbshの実行終了します。nが指定されていない場合は0になります。

### iruby

rbsh内蔵のRubyエンジンでRubyスクリプトを実行します。

⚠現在、irubyコマンドを強制的に終了させる方法はありません。⚠

### puts [arg...]

argを改行コード区切りで出力しします。終了コードは常に0です。

Notes
------

* 現状、bashより実行速度が10倍程度遅いです。

License
--------

MIT License

Authors
--------

* buty4649
  - https://github.com/buty4649/
  - https://twitter.com/buty4649/
  - https://tech.buty4649.info/

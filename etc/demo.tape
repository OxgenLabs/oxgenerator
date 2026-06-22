Output etc/demo.gif

Set Shell "fish"
Set FontSize 18
Set Width 1000
Set Height 820
Set Padding 20
Set Theme "Dracula"

Hide
Type "rm -rf blog-api"
Enter
Type "function fish_prompt; set_color cyan; echo -n '~/dev'; echo; set_color green; echo -n '❯ '; set_color normal; end"
Enter
Type "clear"
Enter
Show

Sleep 700ms

Type "oxgen --help"
Enter
Sleep 4s

Ctrl+L
Sleep 500ms

Type "oxgen new blog-api --database mongodb --dry-run"
Enter
Sleep 4s

Ctrl+L
Sleep 500ms

Type "oxgen new blog-api --database mongodb"
Enter
Sleep 3s

Type "tree -C blog-api -a -I '.git|target'"
Enter
Sleep 4s

Ctrl+L
Sleep 500ms

Type "cd blog-api"
Enter
Sleep 700ms

Type "oxgen generate module post --collection posts --dry-run"
Enter
Sleep 4s

Ctrl+L
Sleep 500ms

Type "oxgen generate module post --collection posts"
Enter
Sleep 3s

Type "tree -C src/modules/post"
Enter
Sleep 4s

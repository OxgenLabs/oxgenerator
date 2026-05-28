Output etc/demo.gif

Set Shell "fish"
Set FontSize 18
Set Width 1000
Set Height 720 
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
Sleep 3s

Ctrl+L
Sleep 500ms

Type "oxgen new blog-api --dry-run"
Enter
Sleep 3s

Ctrl+L
Sleep 500ms

Type "oxgen new blog-api"
Enter
Sleep 2s

Type "tree -C blog-api -a -I '.git|target'"
Enter
Sleep 3s

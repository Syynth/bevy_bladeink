VAR color_change_count = 0

EXTERNAL set_text_color(color)

// define a fallback for your external functions!
== function set_text_color(color)
~ return

== function update_text_color(color)
~ color_change_count += 1
~ set_text_color(color)

== start
~ update_text_color("\#FF0000")
the text is red!
~ update_text_color("\#00FF00")
now the text is green!
~ update_text_color("\#0000FF")
this text is blue!
-> start
// 'round and 'round we go!

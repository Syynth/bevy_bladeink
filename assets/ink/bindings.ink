EXTERNAL set_text_color(color)

// define a fallback for your external functions!
== function set_text_color(color)
~ return

== start
~ set_text_color("\#FF0000")
the text is red!
~ set_text_color("\#00FF00")
now the text is green!
~ set_text_color("\#0000FF")
this text is blue!
-> start
// 'round and 'round we go!

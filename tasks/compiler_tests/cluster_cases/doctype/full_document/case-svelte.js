import * as $ from "svelte/internal/client";
var root = $.from_html(`<!doctype html=""/> <html lang="en"><head><meta charset="utf-8"/> <title>Svelte App</title></head> <body><div>Hello World</div></body></html>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}

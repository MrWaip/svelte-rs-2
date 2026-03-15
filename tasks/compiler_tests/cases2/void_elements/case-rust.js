import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="text"/> <br/> <img src="test.png"/> <hr/>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(6);
	$.append($$anchor, fragment);
}

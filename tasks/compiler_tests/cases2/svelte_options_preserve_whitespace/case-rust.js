import * as $ from "svelte/internal/client";
var root = $.from_html(`

<div>
	hello
	<span>world</span>
	!
</div>`, 1);
export default function App($$anchor) {
	$.next();
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
}

import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class=" tsCompact300XSmall">x</span> <div style="--a: b; ">y</div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}

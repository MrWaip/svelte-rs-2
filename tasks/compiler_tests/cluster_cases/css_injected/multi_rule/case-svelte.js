import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-1aej1md">a</div> <div class="b svelte-1aej1md">b</div>`, 1);
const $$css = {
	hash: "svelte-1aej1md",
	code: ".a.svelte-1aej1md {color:red;}.b.svelte-1aej1md {color:blue;}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}

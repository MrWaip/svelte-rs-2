import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="a svelte-limxtm">a</div> <div class="b svelte-limxtm">b</div>`, 1);
const $$css = {
	hash: "svelte-limxtm",
	code: ".a.svelte-limxtm,\n	.b.svelte-limxtm {color:red;}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var fragment = root();
	$.next(2);
	$.append($$anchor, fragment);
}

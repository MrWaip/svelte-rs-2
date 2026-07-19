import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="box svelte-1ngs2ez">box</div>`);
const $$css = {
	hash: "svelte-1ngs2ez",
	code: "\n	@media (min-width: 100px) {.box.svelte-1ngs2ez {color:red;}\n	}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var div = root();
	$.append($$anchor, div);
}

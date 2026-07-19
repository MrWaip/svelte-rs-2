import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="box svelte-kpvy84">box</div>`);
const $$css = {
	hash: "svelte-kpvy84",
	code: "\n	/* a comment */.box.svelte-kpvy84 {color:red;}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var div = root();
	$.append($$anchor, div);
}

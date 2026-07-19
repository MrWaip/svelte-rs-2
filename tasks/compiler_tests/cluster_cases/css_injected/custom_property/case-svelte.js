import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="box svelte-bulewn">box</div>`);
const $$css = {
	hash: "svelte-bulewn",
	code: ".box.svelte-bulewn {--gap: 10px;color:red;}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var div = root();
	$.append($$anchor, div);
}

import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="used svelte-1n36she">used</div>`);
const $$css = {
	hash: "svelte-1n36she",
	code: ".used.svelte-1n36she {color:red;}.used.svelte-1n36she {border:1px solid;}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var div = root();
	$.append($$anchor, div);
}

import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="box">box</div>`);
const $$css = {
	hash: "svelte-472ibj",
	code: ".box {color:red;}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var div = root();
	$.append($$anchor, div);
}

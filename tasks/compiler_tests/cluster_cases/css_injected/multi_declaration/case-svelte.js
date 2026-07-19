import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="box svelte-354c5f">box</div>`);
const $$css = {
	hash: "svelte-354c5f",
	code: ".box.svelte-354c5f {color:red;width:10px;}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var div = root();
	$.append($$anchor, div);
}

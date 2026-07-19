import * as $ from "svelte/internal/client";
var root = $.from_html(`<span class="icon svelte-p6hspt"></span>`);
const $$css = {
	hash: "svelte-p6hspt",
	code: ".icon.svelte-p6hspt::before {content:\"\\ff\";}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var span = root();
	$.append($$anchor, span);
}

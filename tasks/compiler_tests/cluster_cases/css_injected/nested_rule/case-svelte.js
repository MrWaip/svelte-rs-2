import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="outer svelte-444rfg"><span class="inner svelte-444rfg">inner</span></div>`);
const $$css = {
	hash: "svelte-444rfg",
	code: ".outer.svelte-444rfg {color:red;.inner:where(.svelte-444rfg) {color:blue;}}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var div = root();
	$.append($$anchor, div);
}

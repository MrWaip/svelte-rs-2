import * as $ from "svelte/internal/client";
var root = $.from_html(`<div class="x svelte-11wc98t">x</div>`);
const $$css = {
	hash: "svelte-11wc98t",
	code: "\n    @keyframes svelte-11wc98t-pulse {\n        0% { opacity: 0.4; }\n        100% { opacity: 1; }\n    }.x.svelte-11wc98t { animation: svelte-11wc98t-pulse 1s;}"
};
export default function App($$anchor) {
	$.append_styles($$anchor, $$css);
	var div = root();
	$.append($$anchor, div);
}

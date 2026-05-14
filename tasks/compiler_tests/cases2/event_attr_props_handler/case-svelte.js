import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	var button = root();
	$.delegated("click", button, function(...$$args) {
		$$props.onChange?.apply(this, $$args);
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);

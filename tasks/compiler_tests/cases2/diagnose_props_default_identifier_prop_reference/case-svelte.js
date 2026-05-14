import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	let onSubmit = $.prop($$props, "onSubmit", 19, () => $$props.onClose);
	var button = root();
	$.delegated("click", button, function(...$$args) {
		onSubmit()?.apply(this, $$args);
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);

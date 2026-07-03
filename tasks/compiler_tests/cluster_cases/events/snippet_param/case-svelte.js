import * as $ from "svelte/internal/client";
const row = ($$anchor, handler = $.noop) => {
	var button = root();
	$.delegated("click", button, function(...$$args) {
		handler()?.apply(this, $$args);
	});
	$.append($$anchor, button);
};
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor) {
	row($$anchor, () => () => {});
}
$.delegate(["click"]);

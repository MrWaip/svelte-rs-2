import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>close</button>`);
export default function App($$anchor) {
	const store = { state: { show: true } };
	const close = () => {
		store.state.show = false;
	};
	var button = root();
	$.delegated("click", button, close);
	$.append($$anchor, button);
}
$.delegate(["click"]);

import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $handler = () => $.store_get(handler, "$handler", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const handler = writable();
	$.init();
	var button = root();
	$.event("click", button, function(...$$args) {
		$handler()?.apply(this, $$args);
	});
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}

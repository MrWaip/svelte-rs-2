import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $value = () => $.store_get(value, "$value", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let value = writable({ foo: 1 });
	function reset() {
		$.store_set(value, { foo: 0 });
	}
	$.init();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($value(), $.untrack(() => $value().foo))));
	$.event("click", button, reset);
	$.append($$anchor, button);
	$.pop();
	$$cleanup();
}

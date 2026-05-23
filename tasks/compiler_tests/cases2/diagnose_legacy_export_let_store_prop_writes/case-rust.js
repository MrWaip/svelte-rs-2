import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button> <button>b</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $store = () => $.store_get(store(), "$store", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let store = $.prop($$props, "store", 8);
	function clear() {
		$.store_set(store(), undefined);
	}
	function bump() {
		$.store_mutate(store(), $.untrack($store).x = 1, $.untrack($store));
	}
	$.init();
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.child(button, true);
	$.reset(button);
	var button_1 = $.sibling(button, 2);
	$.template_effect(() => $.set_text(text, ($store(), $.untrack(() => $store()?.x))));
	$.event("click", button, clear);
	$.event("click", button_1, bump);
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}

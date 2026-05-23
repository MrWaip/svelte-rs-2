import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Child from "./Child.svelte";
var root = $.from_html(`<!> <button> </button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $address = () => $.store_get(address, "$address", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let config = $.prop($$props, "config", 8);
	let address = writable(Object.assign({}, config().address));
	function reset() {
		$.store_set(address, { foo: 0 });
	}
	$.init();
	var fragment = root();
	var node = $.first_child(fragment);
	Child(node, {
		get value() {
			return $address().foo;
		},
		set value($$value) {
			$.store_mutate(address, $.untrack($address).foo = $$value, $.untrack($address));
		},
		$$legacy: true
	});
	var button = $.sibling(node, 2);
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($address(), $.untrack(() => $address().foo))));
	$.event("click", button, reset);
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}

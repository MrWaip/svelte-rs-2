import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<!> <button> </button>`, 1), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $address = () => ($.validate_store(address, "address"), $.store_get(address, "$address", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let config = $.prop($$props, "config", 8);
	let address = writable(Object.assign({}, config().address));
	function reset() {
		$.store_set(address, { foo: 0 });
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = root();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => Child(node, {
		get value() {
			return $address().foo;
		},
		set value($$value) {
			$.store_mutate(address, $.untrack($address).foo = $$value, $.untrack($address));
		},
		$$legacy: true
	}), "component", App, 13, 0, { componentTag: "Child" });
	var button = $.sibling(node, 2);
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($address(), $.untrack(() => $address().foo))));
	$.event("click", button, reset);
	$.append($$anchor, fragment);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}

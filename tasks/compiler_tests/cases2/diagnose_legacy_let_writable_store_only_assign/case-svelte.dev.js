import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const $value = () => ($.validate_store(value, "value"), $.store_get(value, "$value", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	let value = writable({ foo: 1 });
	function reset() {
		$.store_set(value, { foo: 0 });
	}
	var $$exports = { ...$.legacy_api() };
	$.init();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, ($value(), $.untrack(() => $value().foo))));
	$.event("click", button, reset);
	$.append($$anchor, button);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}

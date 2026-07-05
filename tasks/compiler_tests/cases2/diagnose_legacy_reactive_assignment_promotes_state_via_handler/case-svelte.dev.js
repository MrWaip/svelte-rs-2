import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button></button>`), App[$.FILENAME], [[15, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let trigger = $.prop($$props, "trigger", 8);
	let value = $.tag($.mutable_source(), "value");
	function read() {
		return $.get(value);
	}
	$.legacy_pre_effect(() => $.deep_read_state(trigger()), () => {
		$.set(value, trigger());
	});
	$.legacy_pre_effect_reset();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.event("click", button, read);
	$.append($$anchor, button);
	return $.pop($$exports);
}

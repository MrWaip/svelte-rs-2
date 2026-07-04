import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 8);
	function foo() {}
	$.legacy_pre_effect(() => $.deep_read_state(a()), () => {
		a();
		foo();
	});
	$.legacy_pre_effect_reset();
	var $$exports = {
		...$.legacy_api(),
		get foo() {
			return foo;
		}
	};
	$.bind_prop($$props, "foo", foo);
	return $.pop($$exports);
}

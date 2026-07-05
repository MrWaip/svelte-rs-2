import "svelte/internal/flags/tracing";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	let doubled = $.tag($.derived(() => {
		return $.trace(() => "$derived.by(...) ((unknown):4:27)", () => {
			return count * 2;
		});
	}), "doubled");
	$.user_effect(() => {
		return $.trace(() => "effect label", () => {
			$.get(doubled);
		});
	});
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}

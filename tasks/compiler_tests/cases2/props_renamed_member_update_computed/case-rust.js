App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let local = $.prop($$props, "value", 23, () => ({ stats: { count: 0 } }));
	let key = "count";
	function bump() {
		$$ownership_validator.mutation("value", [
			"local",
			"stats",
			key
		], local().stats[key]++, 6, 2);
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}

import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Inner from "./Inner.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let value = $.prop($$props, "value", 12, "");
	var $$exports = { ...$.legacy_api() };
	{
		$$ownership_validator.binding("value", Inner, value);
		$.add_svelte_meta(() => Inner($$anchor, {
			get value() {
				return value();
			},
			set value($$value) {
				value($$value);
			},
			$$legacy: true
		}), "component", App, 7, 0, { componentTag: "Inner" });
	}
	return $.pop($$exports);
}

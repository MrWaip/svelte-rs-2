App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let value = $.prop($$props, "value", 7);
	var $$exports = { ...$.legacy_api() };
	{
		$$ownership_validator.binding("value", Child, value);
		$.add_svelte_meta(() => Child($$anchor, {
			get value() {
				return value();
			},
			set value($$value) {
				value($$value);
			}
		}), "component", App, 6, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}

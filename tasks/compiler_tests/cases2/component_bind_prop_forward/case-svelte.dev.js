App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Input from "./Input.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let value = $.prop($$props, "value", 15);
	var $$exports = { ...$.legacy_api() };
	{
		$$ownership_validator.binding("value", Input, value);
		$.add_svelte_meta(() => Input($$anchor, {
			get value() {
				return value();
			},
			set value($$value) {
				value($$value);
			}
		}), "component", App, 5, 0, { componentTag: "Input" });
	}
	return $.pop($$exports);
}

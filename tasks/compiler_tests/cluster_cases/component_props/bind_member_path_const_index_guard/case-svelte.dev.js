App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let value = $.prop($$props, "value", 15);
	const idx = 0;
	var $$exports = { ...$.legacy_api() };
	$.validate_binding("bind:value={value[idx].name}", [], () => value()[idx], () => "name", 7, 7);
	{
		$$ownership_validator.binding("value", Child, () => value()[idx].name);
		$.add_svelte_meta(() => Child($$anchor, {
			get value() {
				return value()[idx].name;
			},
			set value($$value) {
				$$ownership_validator.mutation("value", [
					"value",
					idx,
					"name"
				], value(value()[idx].name = $$value, true), 7, 19);
			}
		}), "component", App, 7, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}

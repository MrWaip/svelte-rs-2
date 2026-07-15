import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let step = $.prop($$props, "step", 12);
	var $$exports = { ...$.legacy_api() };
	$.init();
	{
		$$ownership_validator.binding("step", Child, () => step().editValue);
		$.add_svelte_meta(() => Child($$anchor, {
			get value() {
				return step().editValue;
			},
			set value($$value) {
				$$ownership_validator.mutation(null, ["step", "editValue"], step(step().editValue = $$value, true), 8, 19);
			},
			$$legacy: true
		}), "component", App, 8, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}

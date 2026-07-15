App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Container from "./Container.svelte";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let value = $.prop($$props, "value", 15);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Container($$anchor, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$anchor, $$slotProps) => {
			const idx = $.derived(() => $$slotProps.idx);
			$.validate_binding("bind:value={value[idx].name}", [], () => value()[$.get(idx)], () => "name", 8, 8);
			{
				$$ownership_validator.binding("value", Child, () => value()[$.get(idx)].name);
				$.add_svelte_meta(() => Child($$anchor, {
					get value() {
						return value()[$.get(idx)].name;
					},
					set value($$value) {
						$$ownership_validator.mutation("value", [
							"value",
							$.get(idx),
							"name"
						], value(value()[$.get(idx)].name = $$value, true), 8, 20);
					}
				}), "component", App, 8, 1, { componentTag: "Child" });
			}
		} }
	}), "component", App, 7, 0, { componentTag: "Container" });
	return $.pop($$exports);
}

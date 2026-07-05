App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let foo = $.prop($$props, "foo", 15);
	var $$exports = { ...$.legacy_api() };
	{
		$$ownership_validator.binding("foo", Child, foo);
		$.add_svelte_meta(() => Child($$anchor, {
			get value() {
				return foo();
			},
			set value($$value) {
				foo($$value);
			}
		}), "component", App, 6, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}

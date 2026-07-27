App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	let ref = $.prop($$props, "ref", 15);
	var $$exports = { ...$.legacy_api() };
	{
		$$ownership_validator.binding("ref", Child, ref);
		$.add_svelte_meta(() => Child($$anchor, {
			get ref() {
				return ref();
			},
			set ref($$value) {
				ref($$value);
			},
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				$.next();
				var text = $.text();
				$.template_effect(() => $.set_text(text, $$props.items));
				$.append($$anchor, text);
			}),
			$$slots: { default: true }
		}), "component", App, 6, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}

App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 17]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$ownership_validator = $.create_ownership_validator($$props);
	const foo = $.wrap_snippet(App, function($$anchor, a = $.noop) {
		$.validate_snippet_args(...arguments);
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${$$props.items ?? ""} ${a() ?? ""}`));
		$.append($$anchor, span);
	});
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
				$.add_svelte_meta(() => foo($$anchor, () => 2), "render", App, 8, 16);
			}),
			$$slots: { default: true }
		}), "component", App, 8, 0, { componentTag: "Child" });
	}
	return $.pop($$exports);
}

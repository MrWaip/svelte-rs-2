App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[6, 17]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const foo = $.wrap_snippet(App, function($$anchor, a = $.noop) {
		$.validate_snippet_args(...arguments);
		var span = root();
		var text = $.child(span);
		$.reset(span);
		$.template_effect(() => $.set_text(text, `${$$props.items ?? ""} ${a() ?? ""}`));
		$.append($$anchor, span);
	});
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, {
		children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
			$.add_svelte_meta(() => foo($$anchor, () => 1), "render", App, 8, 7);
		}),
		$$slots: { default: true }
	}), "component", App, 8, 0, { componentTag: "Child" });
	return $.pop($$exports);
}

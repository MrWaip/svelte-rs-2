App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Widget from "./Widget.svelte";
var root = $.add_locations($.from_html(`<span class="inside svelte-1v67kh2">inside</span>`), App[$.FILENAME], [[12, 12]]);
var root_1 = $.add_locations($.from_html(`<div class="host svelte-1v67kh2"><!></div> <span class="inside">outside</span>`, 1), App[$.FILENAME], [[9, 0], [17, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var div = $.first_child(fragment);
	var node = $.child(div);
	{
		const children = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var span = root();
			$.append($$anchor, span);
		});
		$.add_svelte_meta(() => Widget(node, {
			children,
			$$slots: { default: true }
		}), "component", App, 10, 4, { componentTag: "Widget" });
	}
	$.reset(div);
	$.next(2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}

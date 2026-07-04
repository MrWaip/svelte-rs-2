App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Row from "./Row.svelte";
var root = $.add_locations($.from_html(`<p>cell</p>`), App[$.FILENAME], [[14, 2]]);
var root_1 = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[11, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function compute() {
		return $$props.n + 1;
	}
	var $$exports = { ...$.legacy_api() };
	{
		const cell = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var p = root();
			$.append($$anchor, p);
		});
		$.add_svelte_meta(() => Row($$anchor, {
			cell,
			children: $.wrap_snippet(App, ($$anchor, $$slotProps) => {
				const value = $.tag($.derived(compute), "value");
				$.get(value);
				var div = root_1();
				var text = $.child(div, true);
				$.reset(div);
				$.template_effect(() => $.set_text(text, $.get(value)));
				$.append($$anchor, div);
			}),
			$$slots: {
				cell: true,
				default: true
			}
		}), "component", App, 9, 0, { componentTag: "Row" });
	}
	return $.pop($$exports);
}

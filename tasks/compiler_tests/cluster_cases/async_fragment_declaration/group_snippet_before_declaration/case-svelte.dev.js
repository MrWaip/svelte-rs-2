import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<b> </b>`), App[$.FILENAME], [[3, 19]]);
var root_1 = $.add_locations($.from_html(`<div><span> </span> <!></div>`), App[$.FILENAME], [[
	2,
	0,
	[[5, 0]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const id = "name";
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	{
		const greet = $.wrap_snippet(App, function($$anchor, x = $.noop) {
			$.validate_snippet_args(...arguments);
			var b = root();
			var text = $.child(b, true);
			$.reset(b);
			$.template_effect(() => $.set_text(text, x()));
			$.append($$anchor, b);
		});
		let greeting2;
		var promises = $.run([async () => greeting2 = await $.async_derived(async () => (await $.track_reactivity_loss(`Hi ${id}`))(), "greeting2", "(unknown):4:19")]);
		var span = $.child(div);
		var text_1 = $.child(span, true);
		$.reset(span);
		var node = $.sibling(span, 2);
		$.add_svelte_meta(() => greet(node, () => 1), "render", App, 6, 0);
		$.reset(div);
		$.template_effect(() => $.set_text(text_1, $.get(greeting2)), void 0, void 0, [promises[0]]);
	}
	$.append($$anchor, div);
	return $.pop($$exports);
}

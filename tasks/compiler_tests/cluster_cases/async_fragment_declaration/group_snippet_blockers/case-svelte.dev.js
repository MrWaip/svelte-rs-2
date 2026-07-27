import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1> </h1>`), App[$.FILENAME], [[4, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const greet = $.wrap_snippet(App, function($$anchor) {
				$.validate_snippet_args(...arguments);
				var h1 = root();
				var text = $.child(h1, true);
				$.reset(h1);
				$.template_effect(() => $.set_text(text, $.get(number)), void 0, void 0, [promises[0]]);
				$.append($$anchor, h1);
			});
			let number;
			var promises = $.run([async () => number = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve(5)))())))(), "number")]);
			$.add_svelte_meta(() => greet($$anchor), "render", App, 6, 1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 1, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}

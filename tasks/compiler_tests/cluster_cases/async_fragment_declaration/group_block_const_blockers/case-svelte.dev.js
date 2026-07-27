import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>yes</p>`), App[$.FILENAME], [[6, 3]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		const greet = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			let greeting;
			var promises_1 = $.run([async () => greeting = $.tag((await $.save($.async_derived(async () => (await $.save("hi"))())))(), "greeting")]);
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.async(node_1, [promises[0], promises_1[0]], void 0, (node_1) => {
				var consequent = ($$anchor) => {
					var p = root();
					$.append($$anchor, p);
				};
				$.add_svelte_meta(() => $.if(node_1, ($$render) => {
					if ($.get(number) > 4 && $.get(greeting)) $$render(consequent);
				}), "if", App, 5, 2);
			});
			$.append($$anchor, fragment_1);
		});
		let number;
		var promises = $.run([async () => number = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve(5)))())))(), "number")]);
		$.add_svelte_meta(() => greet($$anchor), "render", App, 9, 1);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}

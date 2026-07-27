import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		let number;
		var promises = $.run([async () => number = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve(5)))())))(), "number")]);
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.async(node_1, [promises[0]], void 0, (node_1) => {
			var consequent = ($$anchor) => {
				let length;
				var promises_1 = $.run([() => promises[0].promise, async () => length = $.tag((await $.save($.async_derived(async () => (await $.save($.get(number)))())))(), "length")]);
				var fragment_2 = $.comment();
				var node_2 = $.first_child(fragment_2);
				$.async(node_2, [promises_1[1]], void 0, (node_2) => {
					$.add_svelte_meta(() => $.each(node_2, 17, () => ({ length: $.get(length) }), $.index, ($$anchor, $$item, index) => {
						$.next();
						var text = $.text();
						text.nodeValue = index;
						$.append($$anchor, text);
					}), "each", App, 5, 2);
				});
				$.append($$anchor, fragment_2);
			};
			$.add_svelte_meta(() => $.if(node_1, ($$render) => {
				if ($.get(number) > 4) $$render(consequent);
			}), "if", App, 3, 1);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}

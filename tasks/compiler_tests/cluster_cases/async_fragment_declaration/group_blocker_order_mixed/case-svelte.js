import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>yes</p>`);
export default function App($$anchor) {
	var a;
	var $$promises = $.run([async () => a = await Promise.resolve(1)]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		let b;
		var promises = $.run([async () => b = (await $.save($.async_derived(async () => (await $.save(Promise.resolve(2)))())))()]);
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.async(node_1, [promises[0], $$promises[0]], void 0, (node_1) => {
			var consequent = ($$anchor) => {
				var p = root();
				$.append($$anchor, p);
			};
			$.if(node_1, ($$render) => {
				if ($.get(b) + a > 0) $$render(consequent);
			});
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}

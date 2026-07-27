import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var a;
	var $$promises = $.run([async () => a = await Promise.resolve([1])]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		let b;
		var promises = $.run([async () => b = (await $.save($.async_derived(async () => (await $.save(Promise.resolve([2])))())))()]);
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.async(node_1, [promises[0], $$promises[0]], void 0, (node_1) => {
			$.each(node_1, 17, () => [...$.get(b), ...a], $.index, ($$anchor, x) => {
				var p = root();
				var text = $.child(p, true);
				$.reset(p);
				$.template_effect(() => $.set_text(text, $.get(x)));
				$.append($$anchor, p);
			});
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}

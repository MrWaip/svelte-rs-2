import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		let number;
		var promises = $.run([async () => number = (await $.save($.async_derived(async () => (await $.save(Promise.resolve(5)))())))()]);
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.async(node_1, [promises[0]], void 0, (node_1) => {
			var consequent = ($$anchor) => {
				let length;
				var promises_1 = $.run([() => promises[0].promise, async () => length = (await $.save($.async_derived(async () => (await $.save($.get(number)))())))()]);
				var fragment_2 = $.comment();
				var node_2 = $.first_child(fragment_2);
				$.async(node_2, [promises_1[1]], void 0, (node_2) => {
					$.each(node_2, 17, () => ({ length: $.get(length) }), $.index, ($$anchor, $$item, index) => {
						$.next();
						var text = $.text();
						text.nodeValue = index;
						$.append($$anchor, text);
					});
				});
				$.append($$anchor, fragment_2);
			};
			$.if(node_1, ($$render) => {
				if ($.get(number) > 4) $$render(consequent);
			});
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}

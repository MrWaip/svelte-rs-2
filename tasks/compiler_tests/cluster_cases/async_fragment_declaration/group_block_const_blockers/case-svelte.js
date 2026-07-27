import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>yes</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		const greet = ($$anchor) => {
			let greeting;
			var promises_1 = $.run([async () => greeting = (await $.save($.async_derived(async () => (await $.save("hi"))())))()]);
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.async(node_1, [promises[0], promises_1[0]], void 0, (node_1) => {
				var consequent = ($$anchor) => {
					var p = root();
					$.append($$anchor, p);
				};
				$.if(node_1, ($$render) => {
					if ($.get(number) > 4 && $.get(greeting)) $$render(consequent);
				});
			});
			$.append($$anchor, fragment_1);
		};
		let number;
		var promises = $.run([async () => number = (await $.save($.async_derived(async () => (await $.save(Promise.resolve(5)))())))()]);
		greet($$anchor);
	});
	$.append($$anchor, fragment);
}

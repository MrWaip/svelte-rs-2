import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>a</p>`);
var root_3 = $.from_html(`<p>b</p>`);
var root_4 = $.from_html(`<p>fallback</p>`);
export default function App($$anchor) {
	var a, b;
	var $$promises = $.run([async () => a = await first_fetch(), async () => b = await second_fetch()]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [$$promises[0]], void 0, (node) => {
		var consequent = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		var alternate_1 = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			$.async(node_1, [$$promises[1]], void 0, (node_1) => {
				var consequent_1 = ($$anchor) => {
					var p_1 = root_3();
					$.append($$anchor, p_1);
				};
				var alternate = ($$anchor) => {
					var p_2 = root_4();
					$.append($$anchor, p_2);
				};
				$.if(node_1, ($$render) => {
					if (b) $$render(consequent_1);
					else $$render(alternate, -1);
				}, true);
			});
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (a) $$render(consequent);
			else $$render(alternate_1, -1);
		});
	});
	$.append($$anchor, fragment);
}

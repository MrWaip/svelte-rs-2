import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<span>a</span>`);
var root_3 = $.from_html(`<span>b</span>`);
export default function App($$anchor) {
	let cond = false;
	var fragment = $.comment();
	$.head("q2w0q4", ($$anchor) => {
		$.effect(() => {
			$.document.title = "t";
		});
	});
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var span = root_2();
			$.append($$anchor, span);
		};
		var alternate = ($$anchor) => {
			var span_1 = root_3();
			$.append($$anchor, span_1);
		};
		$.if(node, ($$render) => {
			if (cond) $$render(consequent);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
}

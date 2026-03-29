import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<h1></h1>`);
var root_2 = $.from_html(`<h2></h2>`);
var root_3 = $.from_html(`<h3></h3>`);
var root_4 = $.from_html(`<p></p>`);
export default function App($$anchor) {
	let count = 0;
	let name = "world";
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const label = $.derived(() => name + "!");
			var h1 = root_1();
			h1.textContent = $.get(label);
			$.append($$anchor, h1);
		};
		var consequent_1 = ($$anchor) => {
			var h2 = root_2();
			h2.textContent = "Medium: 0";
			$.append($$anchor, h2);
		};
		var consequent_2 = ($$anchor) => {
			const small = $.derived(() => count * 2);
			var h3 = root_3();
			h3.textContent = `Small doubled: ${$.get(small) ?? ""}`;
			$.append($$anchor, h3);
		};
		var alternate = ($$anchor) => {
			var p = root_4();
			p.textContent = "Tiny: 0";
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if (count > 100) $$render(consequent);
			else if (count > 50) $$render(consequent_1, 1);
			else if (count > 10) $$render(consequent_2, 2);
			else $$render(alternate, -1);
		});
	}
	$.append($$anchor, fragment);
}

App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1></h1>`), App[$.FILENAME], [[8, 4]]);
var root_1 = $.add_locations($.from_html(`<h2></h2>`), App[$.FILENAME], [[10, 4]]);
var root_2 = $.add_locations($.from_html(`<h3></h3>`), App[$.FILENAME], [[13, 4]]);
var root_3 = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[15, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	let name = "world";
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			const label = $.tag($.derived(() => name + "!"), "label");
			$.get(label);
			var h1 = root();
			h1.textContent = $.get(label);
			$.append($$anchor, h1);
		};
		var consequent_1 = ($$anchor) => {
			var h2 = root_1();
			h2.textContent = "Medium: 0";
			$.append($$anchor, h2);
		};
		var consequent_2 = ($$anchor) => {
			const small = $.tag($.derived(() => count * 2), "small");
			$.get(small);
			var h3 = root_2();
			h3.textContent = `Small doubled: ${$.get(small) ?? ""}`;
			$.append($$anchor, h3);
		};
		var alternate = ($$anchor) => {
			var p = root_3();
			p.textContent = "Tiny: 0";
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (count > 100) $$render(consequent);
			else if (count > 50) $$render(consequent_1, 1);
			else if (count > 10) $$render(consequent_2, 2);
			else $$render(alternate, -1);
		}), "if", App, 6, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}

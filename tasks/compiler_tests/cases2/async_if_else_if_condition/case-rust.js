import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>first</p>`);
var root_2 = $.from_html(`<p>second</p>`);
var root_3 = $.from_html(`<p>fallback</p>`);
export default function App($$anchor) {
	async function first() {
		return false;
	}
	async function second() {
		return true;
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.async(node, [], [first], (node, $$condition) => {
		var consequent = ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		};
		var consequent_1 = ($$anchor) => {
			var p_1 = root_2();
			$.append($$anchor, p_1);
		};
		var alternate = ($$anchor) => {
			var p_2 = root_3();
			$.append($$anchor, p_2);
		};
		var d = $.derived(() => await second());
		$.if(node, ($$render) => {
			if ($.get($$condition)) $$render(consequent);
			else if ($.get(d)) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		});
	});
	$.append($$anchor, fragment);
}

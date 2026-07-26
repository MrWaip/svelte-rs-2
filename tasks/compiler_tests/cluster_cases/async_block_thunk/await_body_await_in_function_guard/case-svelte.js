import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>truthy</p>`);
var root_1 = $.from_html(`<p>keyed</p>`);
var root_2 = $.from_html(`<button>inc</button> <!> <!>`, 1);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	function call(callback) {
		return callback();
	}
	var fragment = root_2();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [], [() => call(async () => await delay($.get(x)))], (node, $$condition) => {
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($.get($$condition)) $$render(consequent);
		});
	});
	var node_1 = $.sibling(node, 2);
	$.async(node_1, [], [() => call(async () => await delay($.get(x)))], (node_1, $$key) => {
		$.key(node_1, () => $.get($$key), ($$anchor) => {
			var p_1 = root_1();
			$.append($$anchor, p_1);
		});
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);

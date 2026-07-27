import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>truthy</p>`);
var root_1 = $.from_html(`<p> </p>`);
var root_2 = $.from_html(`<p>keyed</p>`);
var root_3 = $.from_html(`<button>inc</button> <!> <!> <!> <!>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve({
			flag: value,
			list: [value]
		});
	}
	var fragment = root_3();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [], [async () => await (await $.save(delay($.get(x))))().flag], (node, $$condition) => {
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		$.if(node, ($$render) => {
			if ($.get($$condition)) $$render(consequent);
		});
	});
	var node_1 = $.sibling(node, 2);
	$.async(node_1, [], [async () => await (await $.save(delay($.get(x))))().list], (node_1, $$collection) => {
		$.each(node_1, 17, () => $.get($$collection), $.index, ($$anchor, item) => {
			var p_1 = root_1();
			var text = $.child(p_1, true);
			$.reset(p_1);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, p_1);
		});
	});
	var node_2 = $.sibling(node_1, 2);
	$.async(node_2, [], [async () => await (await $.save(delay($.get(x))))().flag], (node_2, $$key) => {
		$.key(node_2, () => $.get($$key), ($$anchor) => {
			var p_2 = root_2();
			$.append($$anchor, p_2);
		});
	});
	var node_3 = $.sibling(node_2, 2);
	$.async(node_3, [], [], (node_3) => {
		$.await(node_3, async () => await (await $.save(delay($.get(x))))().flag, null, ($$anchor, value) => {
			var p_3 = root_1();
			var text_1 = $.child(p_3, true);
			$.reset(p_3);
			$.template_effect(() => $.set_text(text_1, $.get(value)));
			$.append($$anchor, p_3);
		});
	});
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
	$.pop();
}
$.delegate(["click"]);

import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>truthy</p>`), App[$.FILENAME], [[11, 33]]);
var root_1 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[12, 43]]);
var root_2 = $.add_locations($.from_html(`<p>keyed</p>`), App[$.FILENAME], [[13, 34]]);
var root_3 = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[14, 47]]);
var root_4 = $.add_locations($.from_html(`<button>inc</button> <!> <!> <!> <!>`, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	function delay(value) {
		return Promise.resolve({
			flag: value,
			list: [value]
		});
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_4();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [], [async () => (await $.track_reactivity_loss((await $.save(delay($.get(x))))().flag))()], (node, $$condition) => {
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get($$condition)) $$render(consequent);
		}), "if", App, 11, 0);
	});
	var node_1 = $.sibling(node, 2);
	$.async(node_1, [], [async () => (await $.track_reactivity_loss((await $.save(delay($.get(x))))().list))()], (node_1, $$collection) => {
		$.add_svelte_meta(() => $.each(node_1, 17, () => $.get($$collection), $.index, ($$anchor, item) => {
			var p_1 = root_1();
			var text = $.child(p_1, true);
			$.reset(p_1);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, p_1);
		}), "each", App, 12, 0);
	});
	var node_2 = $.sibling(node_1, 2);
	$.async(node_2, [], [async () => (await $.track_reactivity_loss((await $.save(delay($.get(x))))().flag))()], (node_2, $$key) => {
		$.add_svelte_meta(() => $.key(node_2, () => $.get($$key), ($$anchor) => {
			var p_2 = root_2();
			$.append($$anchor, p_2);
		}), "key", App, 13, 0);
	});
	var node_3 = $.sibling(node_2, 2);
	$.async(node_3, [], [], (node_3) => {
		$.add_svelte_meta(() => $.await(node_3, async () => (await $.track_reactivity_loss((await $.save(delay($.get(x))))().flag))(), null, ($$anchor, value) => {
			var p_3 = root_3();
			var text_1 = $.child(p_3, true);
			$.reset(p_3);
			$.template_effect(() => $.set_text(text_1, $.get(value)));
			$.append($$anchor, p_3);
		}), "await", App, 14, 0);
	});
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);

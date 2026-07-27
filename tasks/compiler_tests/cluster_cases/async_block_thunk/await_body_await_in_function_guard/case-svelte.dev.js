import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>truthy</p>`), App[$.FILENAME], [[15, 44]]);
var root_1 = $.add_locations($.from_html(`<p>keyed</p>`), App[$.FILENAME], [[16, 45]]);
var root_2 = $.add_locations($.from_html(`<button>inc</button> <!> <!>`, 1), App[$.FILENAME], [[13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.tag($.state(0), "x");
	function delay(value) {
		return Promise.resolve(value);
	}
	function call(callback) {
		return callback();
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_2();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [], [async () => (await $.track_reactivity_loss(call(async () => (await $.track_reactivity_loss(delay($.get(x))))())))()], (node, $$condition) => {
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get($$condition)) $$render(consequent);
		}), "if", App, 15, 0);
	});
	var node_1 = $.sibling(node, 2);
	$.async(node_1, [], [async () => (await $.track_reactivity_loss(call(async () => (await $.track_reactivity_loss(delay($.get(x))))())))()], (node_1, $$key) => {
		$.add_svelte_meta(() => $.key(node_1, () => $.get($$key), ($$anchor) => {
			var p_1 = root_1();
			$.append($$anchor, p_1);
		}), "key", App, 16, 0);
	});
	$.delegated("click", button, function click() {
		return $.update(x);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);

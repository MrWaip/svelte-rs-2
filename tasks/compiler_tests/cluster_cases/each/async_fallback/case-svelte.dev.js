import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[11, 1]]);
var root_1 = $.add_locations($.from_html(`<p>empty</p>`), App[$.FILENAME], [[13, 1]]);
var root_2 = $.add_locations($.from_html(`<button>inc</button> <!>`, 1), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let n = $.tag($.state(0), "n");
	function delay(value) {
		return Promise.resolve(value);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_2();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [], [async () => (await $.track_reactivity_loss(delay([$.get(n)])))()], (node, $$collection) => {
		$.add_svelte_meta(() => $.each(node, 17, () => $.get($$collection), $.index, ($$anchor, item) => {
			var p = root();
			var text = $.child(p, true);
			$.reset(p);
			$.template_effect(() => $.set_text(text, $.get(item)));
			$.append($$anchor, p);
		}, ($$anchor) => {
			var p_1 = root_1();
			$.append($$anchor, p_1);
		}), "each", App, 10, 0);
	});
	$.delegated("click", button, function click() {
		return $.update(n);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);

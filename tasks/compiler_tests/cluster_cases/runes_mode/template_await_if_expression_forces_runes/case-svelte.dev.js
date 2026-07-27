import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>yes</p>`), App[$.FILENAME], [[10, 1]]);
var root_1 = $.add_locations($.from_html(`<!> <button>inc</button>`, 1), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	async function check(v) {
		return v > 0;
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	$.async(node, [], [async () => (await $.track_reactivity_loss(check(count)))()], (node, $$condition) => {
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get($$condition)) $$render(consequent);
		}), "if", App, 9, 0);
	});
	var button = $.sibling(node, 2);
	$.delegated("click", button, function click() {
		return count++;
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);

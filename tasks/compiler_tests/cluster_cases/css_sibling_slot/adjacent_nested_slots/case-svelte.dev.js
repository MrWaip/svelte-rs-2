import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1 class="svelte-142nm4m">test</h1>`), App[$.FILENAME], [[4, 6]]);
var root_1 = $.add_locations($.from_html(`<span class="svelte-142nm4m">Hello</span>`), App[$.FILENAME], [[10, 4]]);
var root_2 = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_2();
	var node = $.first_child(fragment);
	$.slot(node, $$props, "default", {}, ($$anchor) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.slot(node_1, $$props, "default", {}, ($$anchor) => {
			var fragment_2 = $.comment();
			var node_2 = $.first_child(fragment_2);
			$.slot(node_2, $$props, "default", {}, ($$anchor) => {
				var h1 = root();
				$.append($$anchor, h1);
			});
			$.append($$anchor, fragment_2);
		});
		$.append($$anchor, fragment_1);
	});
	var node_3 = $.sibling(node, 2);
	$.slot(node_3, $$props, "default", {}, ($$anchor) => {
		var fragment_3 = $.comment();
		var node_4 = $.first_child(fragment_3);
		$.slot(node_4, $$props, "default", {}, ($$anchor) => {
			var span = root_1();
			$.append($$anchor, span);
		});
		$.append($$anchor, fragment_3);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}

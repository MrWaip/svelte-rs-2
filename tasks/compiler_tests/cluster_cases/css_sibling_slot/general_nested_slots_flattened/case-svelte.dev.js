import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<h1 class="svelte-11j9i8n">Heading 1</h1>`), App[$.FILENAME], [[1, 12]]);
var root_1 = $.add_locations($.from_html(`<p class="svelte-11j9i8n">Paragraph 2</p>`), App[$.FILENAME], [[1, 94]]);
var root_2 = $.add_locations($.from_html(`<!><span>Span 1</span><span>Span 2</span><!>`, 1), App[$.FILENAME], [[1, 44], [1, 63]]);
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
			var h1 = root();
			$.append($$anchor, h1);
		});
		$.append($$anchor, fragment_1);
	});
	var node_2 = $.sibling(node, 3);
	$.slot(node_2, $$props, "default", {}, ($$anchor) => {
		var fragment_2 = $.comment();
		var node_3 = $.first_child(fragment_2);
		$.slot(node_3, $$props, "default", {}, ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		});
		$.append($$anchor, fragment_2);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}

App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { page } from "$app/stores";
const defaultWrapWith = $.wrap_snippet(App, function($$anchor, mf = $.noop) {
	$.validate_snippet_args(...arguments);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.snippet(node, mf), "render", App, 7, 4);
	$.append($$anchor, fragment);
});
var root = $.add_locations($.from_html(`<b>x</b>`), App[$.FILENAME], [[12, 8]]);
var root_1 = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[16, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const $page = () => ($.validate_store(page, "page"), $.store_get(page, "$page", $$stores));
	const [$$stores, $$cleanup] = $.setup_stores();
	const mf = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		{
			var consequent = ($$anchor) => {
				var b = root();
				$.append($$anchor, b);
			};
			$.add_svelte_meta(() => $.if(node_1, ($$render) => {
				if ($page().url) $$render(consequent);
			}), "if", App, 11, 4);
		}
		$.append($$anchor, fragment_1);
	});
	let wrapWith = $.prop($$props, "wrapWith", 3, defaultWrapWith);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var node_2 = $.child(div);
	$.add_svelte_meta(() => $.snippet(node_2, wrapWith, () => mf), "render", App, 16, 5);
	$.reset(div);
	$.append($$anchor, div);
	var $$pop = $.pop($$exports);
	$$cleanup();
	return $$pop;
}

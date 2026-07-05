App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.with_script($.from_html(`<script async=""><\/script><!>`, 1)), App[$.FILENAME], [[9, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let src = "";
	let cond = false;
	function on_load() {}
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		{
			var consequent = ($$anchor) => {
				var fragment_1 = root();
				var script = $.first_child(fragment_1);
				$.set_attribute(script, "src", src);
				var node_1 = $.sibling(script);
				$.event("load", script, on_load);
				$.replay_events(script);
				$.append($$anchor, fragment_1);
			};
			$.add_svelte_meta(() => $.if(node, ($$render) => {
				if (cond) $$render(consequent);
			}), "if", App, 8, 2);
		}
		$.append($$anchor, fragment);
	});
	return $.pop($$exports);
}

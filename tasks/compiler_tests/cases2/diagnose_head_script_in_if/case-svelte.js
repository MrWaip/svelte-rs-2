import * as $ from "svelte/internal/client";
var root = $.with_script($.from_html(`<script async=""><\/script><!>`, 1));
export default function App($$anchor) {
	let src = "";
	let cond = false;
	function on_load() {}
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
			$.if(node, ($$render) => {
				if (cond) $$render(consequent);
			});
		}
		$.append($$anchor, fragment);
	});
}

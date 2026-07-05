App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.with_script($.from_html(`<script>
        const msg = \`Failed: \${x}\`;
    <\/script><!>`, 1)), App[$.FILENAME], [[2, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root();
		var node = $.sibling($.first_child(fragment));
		$.append($$anchor, fragment);
	});
	return $.pop($$exports);
}

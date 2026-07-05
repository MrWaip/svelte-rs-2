import * as $ from "svelte/internal/client";
var root = $.with_script($.from_html(`<script>
        const msg = \`Failed: \${x}\`;
    <\/script><!>`, 1));
export default function App($$anchor) {
	$.head("q2w0q4", ($$anchor) => {
		var fragment = root();
		var node = $.sibling($.first_child(fragment));
		$.append($$anchor, fragment);
	});
}

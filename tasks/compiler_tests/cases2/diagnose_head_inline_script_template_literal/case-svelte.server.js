import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.push(`<script>
        const msg = \`Failed: \${x}\`;
    <\/script>`);
		$$renderer.push(`<!---->`);
	});
}

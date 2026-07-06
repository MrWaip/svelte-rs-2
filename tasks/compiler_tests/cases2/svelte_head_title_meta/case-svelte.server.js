import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let section = "Dashboard";
	section = "Settings";
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>App - ${$.escape(section)}</title>`);
		});
		$$renderer.push(`<meta charset="utf-8"/> <meta name="description" content="A page"/> <link rel="stylesheet" href="/styles.css"/>`);
	});
}

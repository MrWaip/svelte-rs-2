import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pageTitle = "Home";
	let section = "Dashboard";
	pageTitle = "Other";
	section = "Settings";
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.title(($$renderer) => {
			$$renderer.push(`<title>Static - ${$.escape(pageTitle)} - App ${$.escape(section)}</title>`);
		});
		$$renderer.push(`<meta name="description" content="test"/>`);
	});
}

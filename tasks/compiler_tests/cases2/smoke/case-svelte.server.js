import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<h1><span>Entities ${$.escape(logged_in)}</span> <button>+</button> some long text</h1> <noscript>any content</noscript> `);
	if (!loading) {
		$$renderer.push("<!--[0-->");
		$$renderer.push(`<div>`);
		if (featureA) {
			$$renderer.push("<!--[0-->");
			$$renderer.push(`<div><div>${$.escape(user_name)}</div> <button>${$.escape(counter)}</button></div>`);
		} else if (featureB) {
			$$renderer.push("<!--[1-->");
			$$renderer.push(`<div><p>Lorem</p></div>`);
		} else {
			$$renderer.push("<!--[-1-->");
			$$renderer.push(`<h2>Old UI</h2>`);
		}
		$$renderer.push(`<!--]--></div>`);
	} else {
		$$renderer.push("<!--[-1-->");
		$$renderer.push(`<div>Spinner ${$.escape(percent)}</div>`);
	}
	$$renderer.push(`<!--]-->`);
}

import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let eager = 0;
	let eagerObj = { x: 0 };
	function script_ops() {
		eager = 1;
		eager++;
		eagerObj.x = 2;
		eagerObj.x++;
	}
	$$renderer.push(`<!---->${$.escape(eager)}
${$.escape(eagerObj.x)}
${$.escape(eager = 3)}
${$.escape(eager++)}
${$.escape(eagerObj.x = 4)}
${$.escape(eagerObj.x++)} <button>run</button>`);
}

<script lang="ts">
	export let html: string;
</script>

{@html html}

<svelte:head>
	<style>
		html,
		body,
		#svelte {
			--appBackground: white;
		}

		html,
		body {
			padding: 0;
			margin: 0;
			color: black;
			font-family: 'Times New Roman', Times, serif;
			font-size: 14px;
			line-height: 1.15;
			text-align: justify;
		}

		body {
			max-width: 210mm;
			padding: 1rem;
		}

		p {
			margin: 0;
			text-indent: 12mm;
		}

		h4 {
			margin: 0;
		}

		.logo {
			width: 275px;
		}

		.papper-logo-svg {
			padding-top: 1rem;
		}

		.papper-logo-svg svg {
			fill: #005bff;
		}

		.header {
			display: flex;
			justify-content: flex-end;
			margin-bottom: 18pt;
			text-align: center;
		}

		.header-with-logo {
			align-items: flex-start;
			justify-content: space-between;
			gap: 100px;
			text-align: left;
		}

		.with-top-and-bottom-padding {
			padding: 1rem 0;
		}

		.block {
			flex: 1;
			padding: 0.5rem;
			border: 2px solid black;
			aspect-ratio: 1;
			font-size: 3.2vw;
			font-weight: bold;
		}

		.table-block {
			min-height: 200px;
			padding-right: 0.5rem;
			aspect-ratio: 1 / 1;
			font-size: 16px;
			text-align: center;
		}

		.block:last-child {
			border-left: 0;
		}

		.block.qr {
			display: flex;
			align-items: center;
			justify-content: center;
			border: none;
		}

		.block_small {
			font-size: 2.4vw;
		}

		.title {
			font-weight: bold;
			text-align: center;
		}

		.title-tranche {
			font-size: 18px;
		}

		.date-tranche {
			text-align: right;
		}

		.limit {
			margin: 1.25rem 0;
			text-align: center;
		}

		.title_left {
			text-align: left;
		}

		.title-debt-notify {
			margin: 2rem 0;
			line-height: 2rem;
			text-align: center;
		}

		.info-text {
			margin-top: 1rem;
		}

		.container {
			display: flex;
			width: 100%;
			flex-direction: column;
			gap: 1rem;
		}

		.fullname-block {
			display: flex;
			flex-direction: column;
			align-items: flex-end;
			gap: 0.5rem;
		}

		.heading {
			text-align: center;
		}

		.text-right {
			text-align: right;
		}

		.bordered-block {
			padding: 0.5rem;
			border: 3px solid gray;
		}

		.bottom-margin-medium {
			margin-bottom: 16px;
		}

		.bottom-margin-small {
			margin-bottom: 8px;
		}

		.bottom-margin-large {
			margin-bottom: 24px;
		}

		.text-with-vertical-paddings-small {
			padding: 0.75rem 0;
		}

		.page-break {
			page-break-before: always;
		}

		tr {
			page-break-inside: avoid;
		}

		table,
		th,
		td {
			border: 1px solid black;
			border-collapse: collapse;
			font-size: 1rem;
			vertical-align: top;
		}

		table td {
			padding: 3mm;
		}

		table p {
			text-indent: 0;
		}

		thead td {
			font-weight: bold;
		}

		tr.row {
			break-inside: avoid;
		}

		td.cell {
			padding: 0.5rem;
			border: 1px solid black;
			vertical-align: top;
		}

		.table-tranche {
			width: 100%;
			margin: 1rem 0;
		}

		.subtable {
			padding-left: 2rem;
		}

		.body_full {
			text-indent: 0;
		}

		.subtitle {
			display: flex;
			justify-content: space-between;
			margin-bottom: 1.5rem;
		}

		.signature-block {
			margin-top: 3rem;
		}

		.signature {
			margin-top: 2rem;
		}

		.signature-notify {
			margin-top: 2rem;
			text-align: right;
		}

		.papper-block p + p {
			margin-top: 1rem;
		}

		.table-signature {
			padding: 1rem;
		}

		.tranche-agreement {
			margin-bottom: 1rem;
		}

		.signature-block-notify {
			padding: 0.5rem;
			border: 1px solid black;
		}

		.content {
			display: flex;
			flex-direction: column;
			gap: 1rem;
		}

		.checkbox-wrapper {
			display: flex;
			margin-top: 1rem;
			gap: 4rem;
		}

		.checkbox {
			display: flex;
			gap: 1rem;
		}

		.box {
			display: flex;
			width: 50px;
			height: 50px;
			align-items: center;
			justify-content: center;
			border: solid 1px black;
			font-size: 32px;
		}

		.footer {
			display: flex;
			justify-content: flex-end;
			margin-top: 18pt;
			text-align: center;
		}

		.table-limit {
			width: 100%;
			background-color: white;
			border-collapse: collapse;
		}

		.th,
		.td {
			padding: 10px;
			border: 1px solid black;
			text-align: left;
		}

		.title-limit {
			display: flex;
			flex-direction: column;
			align-items: center;
			margin: 40px 0 60px 0;
		}

		.subtitle-limit {
			margin-bottom: 10px;
			font-size: 14px;
		}
		.title-hr {
			width: 100%;
			border-top: 1px solid #000;
			margin: 3rem 0 1rem 0;
		}

		.header-limit {
			display: flex;
			align-items: flex-start;
			justify-content: space-between;
		}

		.header-limit img {
			width: 200px;
			height: 100px;
		}

		.header-limit .info {
			display: flex;
			flex-direction: column;
			align-items: flex-end;
			padding: 20px;
		}

		.header-limit .info div {
			width: 250px;
			text-align: right;
		}

		.block-with-logo {
			display: grid;
			gap: 0.25rem;
			grid-template-columns: 1fr 2.5fr;
		}

		.table-list {
			margin-bottom: 20px;
		}

		.text-limit {
			margin: 20px 0px 5px 0px;
		}

		.reference {
			padding: 10px;
			border: 1px solid black;
		}

		.notification-header {
			margin-bottom: 20px;
			font-size: 18px;
		}

		.google-info-grid {
			display: flex;
			justify-content: space-between;
			text-align: center;
		}

		.google-info-card--small {
			font-size: 2.4vw;
		}

		.google-info-card {
			flex: 1;
			padding: 0.5rem;
			border: 2px solid black;
			aspect-ratio: 1;
			font-size: 3.2vw;
			font-weight: 700;
			text-align: left;
		}

		.google-info-card--left {
			border-right: none;
		}

		.vertical-center {
			vertical-align: middle;
		}

		@media screen and (min-width: 210mm) {
			.block {
				font-size: 1.9rem;
			}

			.block_small {
				font-size: 1.5rem;
			}
		}

		@media screen and (max-width: 450px) {
			.block {
				font-size: 3.8vw;
			}

			.block.qr {
				flex-grow: 0.4;
				align-items: flex-start;
				padding: 0 5px 0 0;
			}

			.block_small {
				font-size: 3.3vw;
			}

			.logo {
				width: 110px;
			}

			.header-with-logo {
				gap: 16px;
			}
		}

		@media print {
			body {
				font-size: 11pt;
			}

			.signature-block {
				break-inside: avoid;
			}
		}

		.condition-column {
			width: 30ch;
			padding: 0.5rem;
			border: 1px solid black;
		}
	</style>
</svelte:head>

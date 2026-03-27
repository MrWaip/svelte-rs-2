<script>
	let pageTitle = $state("Home");
	let section = $state("Dashboard");
	pageTitle = "Other";
	section = "Settings";
</script>

<svelte:head>
	<title>Static - {pageTitle} - App {section}</title>
	<meta name="description" content="test" />
</svelte:head>

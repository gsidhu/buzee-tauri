<script>
	// import { page } from '$app/stores';
  import { goto } from '$app/navigation';

  // In case of errors, go back to the home page
  goto('/');

  // Not showing error message because this is not a web app
</script>

<!-- <h1>{$page.status} {$page.error?.message}</h1>
<h5>Whoops. Something broke.</h5>
<a href="/">Go back home</a> -->
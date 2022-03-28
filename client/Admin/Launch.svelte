<script>
    import { request } from "../Server/Request.svelte";
  
    // import Request from "../Server/Request.svelte";
    let url = "/admin/launch";
    let promise = request(url);
    let command = "";
    function handleClick() {
      promise = request(url);
    }
  </script>
  

  <main>
    <h1>Launch Application</h1>
    <label for="question">Command:</label>
    <input type="text" size="100" id="command" bind:value={command}>
    <button>Execute</button>
    {#await promise}
      <p>Command: {command}</p>
      <p>...Retrieving result from Rust (with a 1 second sleep)</p>
    {:then text}
      <p>Message from Rust: <pre>{text}</pre>
    {:catch error}
      <p style="color: red">{error.message}</p>
    {/await}
    <button on:click={handleClick}>Refresh</button>
  </main>
  
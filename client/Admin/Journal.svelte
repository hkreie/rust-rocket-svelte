<script>
  import { request } from "../Server/Request.svelte";

  let number = 10;
  let offset = 0;
  let cursor = 0;
  let url = "/admin/journalctl/0/0/10";
  //$: url = "/admin/journalctl/"+cursor+"/"+offset+"/"+number;
  let promise = request(url);
  
  function next() {
      offset += number;
      url = "/admin/journalctl/"+cursor+"/"+offset+"/"+number;
      console.log("url: ", url);
      promise = request(url);
  }
  function prev() {
    if( offset > 0) {
      offset -= number;
      url = "/admin/journalctl/"+cursor+"/"+offset+"/"+number;
      console.log("url: ", url);
      promise = request(url);
    }
  }
  
</script>

<main>
  <h1>Journal</h1>

  <div id="log">
    {#await promise}
    <p>...Retrieving disk space from Rust (with a 1 second sleep)</p>
    {:then text}
    <p>Message from Rust: <br><pre>{text}</pre>
    {:catch error}
    <p style="color: red">{error.message}</p>
    {/await}

    <button on:click={prev}>Prev</button>
    <button on:click={next}>Next</button>

</main>

<style>

</style>
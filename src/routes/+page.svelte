<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";

  let name = "";
  let greetMsg = "";
  let audioDevices: string[] = [];
  let showAudioDevices = false;

  async function greet() {
    greetMsg = await invoke("greet", { name });
  }

  async function fetchAudioDevices(): Promise<string[]> {
    audioDevices = await invoke("show_audio_devices");
    return audioDevices;
  }

  function toggleAudioDevices() {
    showAudioDevices = !showAudioDevices;
    if (showAudioDevices) {
      fetchAudioDevices();
    }
  }
</script>

<main class="container mx-auto px-0 py-10 flex flex-col justify-center items-center text-center h-screen">
  <h1 class="text-3xl font-bold">Welcome to Tauri + Svelte</h1>

  <div class="row flex justify-center mt-4">
    <a href="https://vitejs.dev" target="_blank" class="mx-2 hover:opacity-75">
      <img src="/vite.svg" class="logo vite w-24 h-24 mx-2" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank" class="mx-2 hover:opacity-75">
      <img src="/tauri.svg" class="logo tauri w-24 h-24 mx-2" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank" class="mx-2 hover:opacity-75">
      <img src="/svelte.svg" class="logo svelte-kit w-24 h-24 mx-2" alt="SvelteKit Logo" />
    </a>
  </div>
  <p class="mt-2">Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="flex w-full max-w-sm items-center space-x-2 py-4" on:submit|preventDefault={greet}>
    <Input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <Button type="submit">Greet</Button>
  </form>
  <p class="mt-2">{greetMsg}</p>
  <Button on:click={toggleAudioDevices}>Toggle Audio Devices</Button>
  {#if showAudioDevices}
    <div class="mt-2">
      <ul>
        {#each audioDevices as device}
          <li>{device}</li>
        {/each}
      </ul>
    </div>
  {/if}

</main>

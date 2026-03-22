<script lang="ts">
    import { onMount } from 'svelte';

    import init, { Core, Rom } from 'chip8-emu';

    let core: Core | undefined = $state();
    let rom: Rom | undefined = $state();

    async function getFile(e: Event) {
        const target = e.target as HTMLInputElement;
        const files = target.files!;

        const romFile = files[0];
        const romBuffer = await romFile.arrayBuffer();

        const romName = romFile.name;
        const romData = new Uint8Array(romBuffer);
        const romSize = romFile.size;

        rom = new Rom(romName, romData, romSize);
        console.log(rom);

        core = new Core(rom);
        console.log(core);
    }

    onMount(async () => {
        await init();
    });
</script>

<input type="file" accept=".ch8" onchange={getFile} />

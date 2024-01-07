<script lang="ts">
    import KitHeader from "../../elements/KitHeader.svelte";
    import iLink from "../../icons/link2.svg";
    import iQrCode from "../../icons/qrcode.svg";
    import iCopy from "../../icons/copy.svg";
    import { invoke } from "@tauri-apps/api/tauri";

    let inputValue: string;

    let shortLink: string;
    let qrImage: string;
    let warningMsg: string;

    type Status = "OK" | "Error";

    interface Response {
        status: Status;
        message: string;
        value: any;
    }

    const clear = () => {
        shortLink = "";
        qrImage = "";
        warningMsg = "";
    };

    const get_link = async () => {
        clear();
        const res: Response = await invoke("short_link", { url: inputValue });
        switch (res.status) {
            case "OK":
                shortLink = res.value;
                return;
            case "Error":
                warningMsg = res.value;
                return;
        }
    };

    const get_qrcode = async () => {
        clear();
        const res: string = await invoke("qrcode", { url: inputValue });
        qrImage = res;
    };

    const copyClipboard = (value: string) => {
        navigator.clipboard.writeText(value);
    };
</script>

<KitHeader title="MiniLink" desc="Shorten your link" />
<div class="mb-8">
    <label class="block text-slate-300 font-bold mb-1 text-xl" for="link"
        >Link</label
    >
    <div class="flex gap-1 mb-10">
        <input
            bind:value={inputValue}
            class="px-2 rounded-sm bg-slate-800 w-full h-10 text-lg"
            type="text"
            placeholder="Put your link here"
        />
        <button
            on:click={get_link}
            class="px-2 bg-slate-800 hover:bg-slate-800/50 w-20 h-10 text-lg rounded-sm"
        >
            <img class="m-auto" src={iLink} alt="icon" />
        </button>
        <button
            on:click={get_qrcode}
            class="px-2 bg-slate-800 hover:bg-slate-800/50 w-20 h-10 text-lg rounded-sm"
        >
            <img class="m-auto" src={iQrCode} alt="icon" />
        </button>
    </div>
    {#if warningMsg}
        <label class="bg-rose-500/40 text-slate-200 p-1 rounded" for="warning">
            {warningMsg}
        </label>
    {/if}
</div>

{#if shortLink}
    <div class="flex gap-1 mb-10">
        <input
            bind:value={shortLink}
            class="px-2 rounded-sm text-slate-600 bg-slate-800/50 w-1/2 h-10 text-lg"
            type="text"
            disabled
        />
        <button
            on:click={() => copyClipboard(shortLink)}
            class="px-2 bg-slate-800 hover:bg-slate-800/50 w-20 h-10 text-lg rounded-sm"
        >
            <img class="m-auto" src={iCopy} alt="icon" />
        </button>
    </div>
{/if}

{#if qrImage}
    <div class="bg-slate-800 w-fit p-3 rounded m-auto">
        <img class="mx-auto" src={qrImage} id="qrcode" alt="qrcode" />
    </div>
{/if}

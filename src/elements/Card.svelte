<script lang="ts">
    export let title: string;
    export let icon: string;
</script>

<div class="card-element">
    <label class="lcard-elem" for="header">{title}</label>
    <div class="h-full w-full">
        <img class="icon" src={icon} alt="icon" />
    </div>
</div>

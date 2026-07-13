<script lang="ts">
  export let onClick: () => void = () => {}
</script>

<button class="trigger" on:click={onClick} title="Keyboard shortcuts (?)">
  <span>?</span>
</button>

<style>
.trigger {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 30;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: 1px solid #EAEAEA;
  background: #FFFFFF;
  color: #787774;
  font-size: 14px;
  font-family: 'Geist Mono', 'SF Mono', monospace;
  font-weight: 500;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  transition: border-color 120ms, color 120ms, transform 80ms;
}

.trigger:hover {
  border-color: #111111;
  color: #111111;
  transform: scale(1.05);
}

.trigger:active {
  transform: scale(0.96);
}
</style>

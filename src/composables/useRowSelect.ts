import { ref, onMounted, onUnmounted } from 'vue'

export function useRowSelect(lengthRef: () => number, modalOpenRef: () => boolean) {
  const selectedIndex = ref<number | null>(null)

  const selectNext = () => {
    if (lengthRef() === 0 || modalOpenRef()) return
    if (selectedIndex.value === null) selectedIndex.value = 0
    else selectedIndex.value = Math.min(selectedIndex.value + 1, lengthRef() - 1)
    console.log(`length ref: ${lengthRef()}, selectedIndex: ${selectedIndex.value}`)
  }

  const selectPrev = () => {
    if (lengthRef() === 0 || modalOpenRef()) return
    if (selectedIndex.value === null) selectedIndex.value = 0
    else selectedIndex.value = Math.max(selectedIndex.value - 1, 0)
    console.log(`length ref: ${lengthRef()}, selectedIndex: ${selectedIndex.value}`)
  }

  const resetSelectedIndex = () => {
    selectedIndex.value = null
  }

  const handleKeydown = (e: KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      selectNext()
      //   e.preventDefault();
    }
    if (e.key === 'ArrowUp') {
      selectPrev()
      //   e.preventDefault();
    }
  }

  onMounted(() => window.addEventListener('keydown', handleKeydown))
  onUnmounted(() => window.removeEventListener('keydown', handleKeydown))

  return { selectedIndex, selectNext, selectPrev, resetSelectedIndex }
}

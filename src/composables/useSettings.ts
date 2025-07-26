import { ref } from 'vue'

export const useSettings = () => {
  const displaySettingsModal = ref(false)

  const toggleSettingsModal = () => {
    displaySettingsModal.value = !displaySettingsModal.value
  }

  return {
    displaySettingsModal,
    toggleSettingsModal
  }
}

<template>
  <div class="home">
    <v-img
      src="../assets/home.png"
    />
    <notice-board />
  </div>
</template>

<script lang="ts">
  import NoticeBoard from '@/components/NoticeBoard.vue'

  export default {
    name: 'MuonHome',
    components: {
      NoticeBoard,
    },
  }
</script>

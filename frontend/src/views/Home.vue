<template>
  <div class="home">
    <v-img
      src="../assets/home.png"
    />
    <notice-board />
  </div>
</template>

<script>
  import NoticeBoard from '@/components/NoticeBoard.vue'

  export default {
    name: 'Home',
    components: {
      NoticeBoard,
    },
  }
</script>

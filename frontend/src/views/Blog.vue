<template>
  <div>
    <ImageHeader
      :src="require('../assets/blog.jpg')"
      title="Blog"
      title-color="darkblue"
    />
    <v-container />
  </div>
</template>

<script lang="ts">
  import { Component, Vue } from 'vue-property-decorator'
  import ImageHeader from '@/components/ImageHeader.vue'

  @Component({
    components: {
      ImageHeader,
    },
  })
  export default class Blog extends Vue {
  }
</script>

<style lang="scss" scoped>
</style>

<template>
  <div>
    <GmapMap
      :center="{lat:37.537990, lng:126.892892}"
      :zoom="18"
      style="width: 100%; height: 400px"
    >
      <GmapMarker
        :position="{lat:37.538088, lng:126.891587}"
        :clickable="true"
        :draggable="true"
        @click="center={lat:37.891587, lng:126.891587}"
      />
    </GmapMap>
  </div>
</template>

<script>
  import { Component, Vue } from 'vue-property-decorator'

  @Component({})
  export default class Map extends Vue {
  }
</script>

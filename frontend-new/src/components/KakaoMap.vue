<template>
  <div id="map">
  </div>
</template>

<style scoped>
#map {
  width: 100%;
  height: 400px;
}
</style>
<script lang="ts">
import { Component, Vue } from 'vue-property-decorator'

@Component({})
export default class KakaoMap extends Vue {
  map: any = null

  mounted() {
    if ((window as any).kakao && (window as any).kakao.maps) {
      this.loadMap()
    }
  }

  loadMap() {
    const container = document.getElementById("map");
    const options = {
      center: new (window as any).kakao.maps.LatLng(37.537990, 126.892892),
      level: 3,
    }

    this.map = new (window as any).kakao.maps.Map(container, options)
    this.loadMarker()
  }

  loadMarker() {
    const markerPosition = new (window as any).kakao.maps.LatLng(37.53823, 126.89159)

    const marker = new (window as any).kakao.maps.Marker({
      map: this.map,
      position: markerPosition,
      title: "뮤온(주)"
    })
  }
}
</script>
  
<template>
  <v-parallax
    :src="src"
    height="200"
  >
    <v-row
      align="center"
      justify="center"
    >
      <v-col
        class="text-left ml-10"
        cols="12"
      >
        <v-container>
          <v-row>
            <h1
              class="display-1 font-weight-bold"
              :style="{'color': titleColor}"
            >
              {{ title }}
            </h1>
          </v-row>
        </v-container>
      </v-col>
    </v-row>
  </v-parallax>
</template>

<script lang="ts">
  import { Vue, Component, Prop } from 'vue-property-decorator'

  @Component({})
  export default class ImageHeader extends Vue {
    @Prop() src!: string
    @Prop() title!: string
    @Prop() titleColor!: string
  }
</script>
